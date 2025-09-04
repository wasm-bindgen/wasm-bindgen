//! Because some WebIDL constructs are defined in multiple places
//! (keyword `partial` is used to add to an existing construct),
//! We need to first walk the webidl to collect all non-partial
//! constructs so that we have containers in which to put the
//! partial ones.
//!
//! Only `interface`s, `dictionary`s, `enum`s and `mixin`s can
//! be partial.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use lazy_static::lazy_static;
use weedle::argument::Argument;
use weedle::attribute::*;
use weedle::common::{Identifier, Punctuated};
use weedle::interface::*;
use weedle::mixin::*;
use weedle::namespace::*;
use weedle::term;
use weedle::types::*;
use weedle::CallbackInterfaceDefinition;
use weedle::{DictionaryDefinition, PartialDictionaryDefinition};

use super::Result;
use crate::{
    util::{self, camel_case_ident},
    ApiStability,
};

lazy_static! {
    // [Throws]
    static ref THROWS_ATTR: Option<ExtendedAttributeList<'static>> = {
        Some(ExtendedAttributeList {
            open_bracket: term::OpenBracket,
            body: Punctuated {
                list: vec![ExtendedAttribute::NoArgs(ExtendedAttributeNoArgs(
                    Identifier("Throws"),
                ))],
                separator: term!(,),
            },
            close_bracket: term::CloseBracket,
        })
    };

    // [NewObject]
    static ref NEW_OBJECT_ATTR: Option<ExtendedAttributeList<'static>> = {
        Some(ExtendedAttributeList {
            open_bracket: term::OpenBracket,
            body: Punctuated {
                list: vec![ExtendedAttribute::NoArgs(ExtendedAttributeNoArgs(
                    Identifier("NewObject"),
                ))],
                separator: term!(,),
            },
            close_bracket: term::CloseBracket,
        })
    };
}

/// Collection of constructs that may use partial.
#[derive(Default)]
pub(crate) struct FirstPassRecord<'src> {
    pub(crate) interfaces: BTreeMap<&'src str, InterfaceData<'src>>,
    pub(crate) enums: BTreeMap<&'src str, EnumData<'src>>,
    /// The mixins, mapping their name to the webidl ast node for the mixin.
    pub(crate) mixins: BTreeMap<&'src str, MixinData<'src>>,
    pub(crate) typedefs: BTreeMap<&'src str, &'src weedle::types::Type<'src>>,
    pub(crate) namespaces: BTreeMap<&'src str, NamespaceData<'src>>,
    pub(crate) includes: BTreeMap<&'src str, BTreeSet<&'src str>>,
    pub(crate) dictionaries: BTreeMap<&'src str, DictionaryData<'src>>,
    pub(crate) callbacks: BTreeSet<&'src str>,
    pub(crate) iterators: BTreeSet<&'src str>,
    pub(crate) async_iterators: BTreeSet<&'src str>,
    pub(crate) callback_interfaces: BTreeMap<&'src str, CallbackInterfaceData<'src>>,
}

pub(crate) struct AttributeInterfaceData<'src> {
    pub(crate) definition: &'src AttributeInterfaceMember<'src>,
    pub(crate) stability: ApiStability,
}

pub(crate) struct ConstData<'src> {
    pub(crate) definition: &'src ConstMember<'src>,
    pub(crate) stability: ApiStability,
}

#[derive(Clone)]
pub(crate) struct ConstNamespaceData<'src> {
    pub(crate) definition: &'src ConstNamespaceMember<'src>,
    pub(crate) stability: ApiStability,
}

/// We need to collect interface data during the first pass, to be used later.
#[derive(Default)]
pub(crate) struct InterfaceData<'src> {
    /// Whether only partial interfaces were encountered
    pub(crate) partial: bool,
    pub(crate) has_interface: bool,
    pub(crate) deprecated: Option<Option<String>>,
    pub(crate) attributes: Vec<AttributeInterfaceData<'src>>,
    pub(crate) consts: Vec<ConstData<'src>>,
    pub(crate) operations: BTreeMap<OperationId<'src>, OperationData<'src>>,
    pub(crate) superclass: Option<&'src str>,
    pub(crate) definition_attributes: Option<&'src ExtendedAttributeList<'src>>,
    pub(crate) stability: ApiStability,
}

pub(crate) struct AttributeMixinData<'src> {
    pub(crate) definition: &'src AttributeMixinMember<'src>,
    pub(crate) stability: ApiStability,
}

/// We need to collect mixin data during the first pass, to be used later.
#[derive(Default)]
pub(crate) struct MixinData<'src> {
    /// Whether only partial mixins were encountered
    pub(crate) partial: bool,
    pub(crate) attributes: Vec<AttributeMixinData<'src>>,
    pub(crate) consts: Vec<&'src ConstMember<'src>>,
    pub(crate) operations: BTreeMap<OperationId<'src>, OperationData<'src>>,
    pub(crate) definition_attributes: Option<&'src ExtendedAttributeList<'src>>,
    pub(crate) stability: ApiStability,
}

/// We need to collect namespace data during the first pass, to be used later.
#[derive(Default)]
pub(crate) struct NamespaceData<'src> {
    pub(crate) operations: BTreeMap<OperationId<'src>, OperationData<'src>>,
    pub(crate) consts: Vec<ConstNamespaceData<'src>>,
    pub(crate) stability: ApiStability,
}

pub(crate) struct PartialDictionaryData<'src> {
    pub(crate) definition: &'src PartialDictionaryDefinition<'src>,
    pub(crate) stability: ApiStability,
}

#[derive(Default)]
pub(crate) struct DictionaryData<'src> {
    pub(crate) partials: Vec<PartialDictionaryData<'src>>,
    pub(crate) definition: Option<&'src DictionaryDefinition<'src>>,
    pub(crate) stability: ApiStability,
}

pub(crate) struct EnumData<'src> {
    pub(crate) definition: &'src weedle::EnumDefinition<'src>,
    pub(crate) stability: ApiStability,
}

pub(crate) struct CallbackInterfaceData<'src> {
    pub(crate) definition: &'src CallbackInterfaceDefinition<'src>,
    pub(crate) single_function: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub(crate) enum OperationId<'src> {
    /// The name of a constructor in crates/web-sys/webidls/enabled/*.webidl
    ///
    /// ex: Constructor(Some("ImageData"))
    Constructor(Option<&'src str>),
    NamedConstructor(IgnoreTraits<&'src str>),
    /// The name of a function in crates/web-sys/webidls/enabled/*.webidl
    ///
    /// ex: Operation(Some("vertexAttrib1fv"))
    Operation(Option<&'src str>),
    IndexingGetter,
    IndexingSetter,
    IndexingDeleter,
}

#[derive(Default)]
pub(crate) struct OperationData<'src> {
    pub(crate) signatures: Vec<Signature<'src>>,
    pub(crate) is_static: bool,
    pub(crate) stability: ApiStability,
}

#[derive(Clone, Debug)]
pub(crate) struct Signature<'src> {
    pub(crate) args: Vec<Arg<'src>>,
    pub(crate) ret: weedle::types::ReturnType<'src>,
    pub(crate) attrs: &'src Option<ExtendedAttributeList<'src>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Arg<'src> {
    pub(crate) attributes: &'src Option<ExtendedAttributeList<'src>>,
    pub(crate) name: &'src str,
    pub(crate) ty: &'src weedle::types::Type<'src>,
    pub(crate) optional: bool,
    pub(crate) variadic: bool,
}

impl<'a> From<&'a Argument<'a>> for Arg<'a> {
    fn from(arg: &'a Argument<'a>) -> Self {
        let (attributes, name, ty, optional, variadic) = match arg {
            Argument::Single(single) => (
                &single.attributes,
                single.identifier.0,
                &single.type_.type_,
                single.optional.is_some(),
                false,
            ),
            Argument::Variadic(variadic) => (
                &variadic.attributes,
                variadic.identifier.0,
                &variadic.type_,
                false,
                true,
            ),
        };

        Self {
            attributes,
            name,
            ty,
            optional,
            variadic,
        }
    }
}

/// Implemented on an AST node to populate the `FirstPassRecord` struct.
pub(crate) trait FirstPass<'src, Ctx> {
    /// Populate `record` with any constructs in `self`.
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, ctx: Ctx) -> Result<()>;
}

impl<'src> FirstPass<'src, ApiStability> for [weedle::Definition<'src>] {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        stability: ApiStability,
    ) -> Result<()> {
        for def in self {
            def.first_pass(record, stability)?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, ApiStability> for weedle::Definition<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        stability: ApiStability,
    ) -> Result<()> {
        use weedle::Definition::*;

        match self {
            Dictionary(dictionary) => dictionary.first_pass(record, stability),
            PartialDictionary(dictionary) => dictionary.first_pass(record, stability),
            Enum(enum_) => enum_.first_pass(record, stability),
            IncludesStatement(includes) => includes.first_pass(record, ()),
            Interface(interface) => interface.first_pass(record, stability),
            PartialInterface(interface) => interface.first_pass(record, stability),
            InterfaceMixin(mixin) => mixin.first_pass(record, stability),
            PartialInterfaceMixin(mixin) => mixin.first_pass(record, stability),
            Namespace(namespace) => namespace.first_pass(record, stability),
            PartialNamespace(namespace) => namespace.first_pass(record, stability),
            Typedef(typedef) => typedef.first_pass(record, ()),
            Callback(callback) => callback.first_pass(record, ()),
            CallbackInterface(iface) => iface.first_pass(record, ()),
            Implements(_) => Ok(()),
        }
    }
}

impl<'src> FirstPass<'src, ApiStability> for weedle::DictionaryDefinition<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        stability: ApiStability,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        let dictionary_data = record.dictionaries.entry(self.identifier.0).or_default();

        dictionary_data.definition = Some(self);
        dictionary_data.stability = stability;

        Ok(())
    }
}

impl<'src> FirstPass<'src, ApiStability> for weedle::PartialDictionaryDefinition<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        stability: ApiStability,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        record
            .dictionaries
            .entry(self.identifier.0)
            .or_default()
            .partials
            .push(PartialDictionaryData {
                definition: self,
                stability,
            });

        Ok(())
    }
}

impl<'src> FirstPass<'src, ApiStability> for weedle::EnumDefinition<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        stability: ApiStability,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        let enum_data = EnumData {
            definition: self,
            stability,
        };

        if record.enums.insert(self.identifier.0, enum_data).is_some() {
            log::info!(
                "Encountered multiple enum declarations: {}",
                self.identifier.0
            );
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::IncludesStatementDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        record
            .includes
            .entry(self.lhs_identifier.0)
            .or_default()
            .insert(self.rhs_identifier.0);

        Ok(())
    }
}

#[derive(Clone, Copy)]
enum FirstPassOperationType {
    Interface,
    Mixin,
    Namespace,
}

fn first_pass_operation<'src, A: Into<Arg<'src>> + 'src>(
    record: &mut FirstPassRecord<'src>,
    first_pass_operation_type: FirstPassOperationType,
    self_name: &'src str,
    ids: &[OperationId<'src>],
    arguments: impl IntoIterator<Item = A>,
    ret: &weedle::types::ReturnType<'src>,
    attrs: &'src Option<ExtendedAttributeList<'src>>,
    is_static: bool,
    stability: ApiStability,
) {
    if util::is_chrome_only(attrs) {
        return;
    }

    let operations = match first_pass_operation_type {
        FirstPassOperationType::Interface => {
            let x = record
                .interfaces
                .get_mut(self_name)
                .unwrap_or_else(|| panic!("not found {self_name} interface"));
            &mut x.operations
        }
        FirstPassOperationType::Mixin => {
            let x = record
                .mixins
                .get_mut(self_name)
                .unwrap_or_else(|| panic!("not found {self_name} mixin"));
            &mut x.operations
        }
        FirstPassOperationType::Namespace => {
            let x = record
                .namespaces
                .get_mut(self_name)
                .unwrap_or_else(|| panic!("not found {self_name} namespace"));
            &mut x.operations
        }
    };
    let args = arguments.into_iter().map(Into::into).collect::<Vec<_>>();
    for id in ids {
        let op = operations.entry(*id).or_default();
        op.is_static = is_static;
        op.stability = stability;
        op.signatures.push(Signature {
            args: args.clone(),
            ret: ret.clone(),
            attrs,
        });
    }
}

impl<'src> FirstPass<'src, ApiStability> for weedle::InterfaceDefinition<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        stability: ApiStability,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        let interface_data = record.interfaces.entry(self.identifier.0).or_default();
        interface_data.partial = false;
        interface_data.superclass = self.inheritance.map(|s| s.identifier.0);
        interface_data.definition_attributes = self.attributes.as_ref();
        interface_data.deprecated = util::get_rust_deprecated(&self.attributes);
        interface_data.has_interface = !util::is_no_interface_object(&self.attributes);
        interface_data.stability = stability;
        if let Some(attrs) = &self.attributes {
            for attr in attrs.body.list.iter() {
                process_interface_attribute(record, self.identifier.0, attr);
            }
        }

        for member in &self.members.body {
            member.first_pass(record, (self.identifier.0, stability))?;
        }

        Ok(())
    }
}

fn process_interface_attribute<'src>(
    record: &mut FirstPassRecord<'src>,
    self_name: &'src str,
    attr: &'src ExtendedAttribute<'src>,
) {
    let stability = record.interfaces[self_name].stability;
    let ident = weedle::common::Identifier(self_name);
    let non_null = weedle::types::MayBeNull {
        type_: ident,
        q_mark: None,
    };
    let non_any = weedle::types::NonAnyType::Identifier(non_null);
    let single = weedle::types::SingleType::NonAny(non_any);
    let ty = weedle::types::Type::Single(single);
    let return_ty = weedle::types::ReturnType::Type(ty);
    match attr {
        ExtendedAttribute::ArgList(list) if list.identifier.0 == "Constructor" => {
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                self_name,
                &[OperationId::Constructor(Some(self_name))],
                &list.args.body.list,
                &return_ty,
                &None,
                false,
                stability,
            );
        }
        ExtendedAttribute::NoArgs(other) if (other.0).0 == "Constructor" => {
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                self_name,
                &[OperationId::Constructor(Some(self_name))],
                &[],
                &return_ty,
                &None,
                false,
                stability,
            );
        }
        ExtendedAttribute::NamedArgList(list) if list.lhs_identifier.0 == "NamedConstructor" => {
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                self_name,
                &[OperationId::NamedConstructor(IgnoreTraits(
                    list.rhs_identifier.0,
                ))],
                &list.args.body.list,
                &return_ty,
                &None,
                false,
                stability,
            );
        }
        _ => {}
    }
}

impl<'src> FirstPass<'src, ApiStability> for weedle::PartialInterfaceDefinition<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        stability: ApiStability,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }
        record
            .interfaces
            .entry(self.identifier.0)
            .or_insert_with(|| InterfaceData {
                partial: true,
                stability,
                ..Default::default()
            });
        for member in &self.members.body {
            member.first_pass(record, (self.identifier.0, stability))?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)> for weedle::interface::InterfaceMember<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        ctx: (&'src str, ApiStability),
    ) -> Result<()> {
        match self {
            InterfaceMember::Attribute(attr) => attr.first_pass(record, ctx),
            InterfaceMember::Operation(op) => op.first_pass(record, ctx),
            InterfaceMember::Const(const_) => {
                if util::is_chrome_only(&const_.attributes) {
                    return Ok(());
                }
                record
                    .interfaces
                    .get_mut(ctx.0)
                    .unwrap()
                    .consts
                    .push(ConstData {
                        definition: const_,
                        stability: ctx.1,
                    });
                Ok(())
            }
            InterfaceMember::Constructor(constr) => constr.first_pass(record, ctx),
            InterfaceMember::Maplike(ml) => ml.first_pass(record, ctx),
            InterfaceMember::Setlike(sl) => sl.first_pass(record, ctx),
            InterfaceMember::Iterable(iterable) => iterable.first_pass(record, ctx),
            InterfaceMember::AsyncIterable(iterable) => iterable.first_pass(record, ctx),
            InterfaceMember::Stringifier(_) => {
                log::warn!("Unsupported WebIDL Stringifier interface member: {self:?}");
                Ok(())
            }
        }
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)>
    for weedle::interface::OperationInterfaceMember<'src>
{
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        (self_name, stability): (&'src str, ApiStability),
    ) -> Result<()> {
        let is_static = match self.modifier {
            Some(StringifierOrStatic::Stringifier(_)) => {
                log::warn!("Unsupported webidl stringifier: {self:?}");
                return Ok(());
            }
            Some(StringifierOrStatic::Static(_)) => true,
            None => false,
        };

        let mut ids = vec![OperationId::Operation(self.identifier.map(|s| s.0))];
        if let Some(special) = self.special {
            match special {
                Special::Getter(_) => ids.push(OperationId::IndexingGetter),
                Special::Setter(_) => ids.push(OperationId::IndexingSetter),
                Special::Deleter(_) => ids.push(OperationId::IndexingDeleter),
                Special::LegacyCaller(_) => {}
            };
        }
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            self_name,
            &ids,
            &self.args.body.list,
            &self.return_type,
            &self.attributes,
            is_static,
            stability,
        );
        Ok(())
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)>
    for weedle::interface::ConstructorInterfaceMember<'src>
{
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        (self_name, stability): (&'src str, ApiStability),
    ) -> Result<()> {
        let ident = weedle::common::Identifier(self_name);
        let non_null = weedle::types::MayBeNull {
            type_: ident,
            q_mark: None,
        };
        let non_any = weedle::types::NonAnyType::Identifier(non_null);
        let single = weedle::types::SingleType::NonAny(non_any);
        let ty = weedle::types::Type::Single(single);
        let return_ty = weedle::types::ReturnType::Type(ty);

        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            self_name,
            &[OperationId::Constructor(Some(self_name))],
            &self.args.body.list,
            &return_ty,
            &self.attributes,
            false,
            stability,
        );

        Ok(())
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)>
    for weedle::interface::MaplikeInterfaceMember<'src>
{
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        (self_name, stability): (&'src str, ApiStability),
    ) -> Result<()> {
        let key_ty = &self.generics.body.0;
        let value_ty = &self.generics.body.2;
        let key_arg = || Arg {
            attributes: &None,
            name: "key",
            ty: &key_ty.type_,
            optional: false,
            variadic: false,
        };
        let value_arg = || Arg {
            attributes: &None,
            name: "value",
            ty: &value_ty.type_,
            optional: false,
            variadic: false,
        };
        let opt_value_ret = || ReturnType::Type(util::nullable(value_ty.type_.clone()));
        let undefined_ret = || ReturnType::Undefined(term!(undefined));

        // readonly attribute unsigned long size;
        record
            .interfaces
            .get_mut(self_name)
            .unwrap()
            .attributes
            .push(AttributeInterfaceData {
                definition: &AttributeInterfaceMember {
                    attributes: None,
                    modifier: None,
                    readonly: Some(term!(readonly)),
                    attribute: term!(attribute),
                    type_: AttributedType {
                        attributes: None,
                        type_: Type::Single(SingleType::NonAny(NonAnyType::Integer(MayBeNull {
                            type_: IntegerType::Long(LongType {
                                unsigned: Some(term!(unsigned)),
                                long: term!(long),
                            }),
                            q_mark: None,
                        }))),
                    },
                    identifier: Identifier("size"),
                    semi_colon: term!(;),
                },
                stability,
            });

        // boolean has(K key);
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            self_name,
            &[OperationId::Operation(Some("has"))],
            [key_arg()],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Boolean(
                MayBeNull {
                    type_: term!(boolean),
                    q_mark: None,
                },
            )))),
            &None,
            false,
            stability,
        );

        // V? get(K key);
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            self_name,
            &[OperationId::Operation(Some("get"))],
            [key_arg()],
            &opt_value_ret(),
            &None,
            false,
            stability,
        );

        // callback MapLikeForEachCallback = undefined (V value, K key);
        // TODO: the signature of the callback is erased, could we keep it?
        let foreach_callback_arg = Arg {
            attributes: &None,
            name: "callback",
            ty: &Type::Single(SingleType::NonAny(NonAnyType::Identifier(MayBeNull {
                type_: Identifier("MapLikeForEachCallback"),
                q_mark: None,
            }))),
            optional: false,
            variadic: false,
        };

        record.callbacks.insert("MapLikeForEachCallback");

        // [Throws] undefined forEach(MapLikeForEachCallback cb);
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            self_name,
            &[OperationId::Operation(Some("forEach"))],
            [foreach_callback_arg],
            &undefined_ret(),
            &THROWS_ATTR,
            false,
            stability,
        );

        // TODO: iterators could have stronger types by generating specialised interfaces for each
        //       maplike/setlike. Right now, `value` is always `any`.

        // declare the iterator interface
        record.iterators.insert("MapLikeIterator");

        // [NewObject] MapLikeIterator entries();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            self_name,
            &[OperationId::Operation(Some("entries"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("MapLikeIterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            stability,
        );

        // [NewObject] MapLikeIterator keys();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            self_name,
            &[OperationId::Operation(Some("keys"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("MapLikeIterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            stability,
        );

        // [NewObject] MapLikeIterator values();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            self_name,
            &[OperationId::Operation(Some("values"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("MapLikeIterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            stability,
        );

        // add writeable interface if *not* readonly
        if self.readonly.is_none() {
            // undefined clear();
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                self_name,
                &[OperationId::Operation(Some("clear"))],
                &[],
                &undefined_ret(),
                &None,
                false,
                stability,
            );

            // boolean delete(K key);
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                self_name,
                &[OperationId::Operation(Some("delete"))],
                [key_arg()],
                &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Boolean(
                    MayBeNull {
                        type_: term!(boolean),
                        q_mark: None,
                    },
                )))),
                &None,
                false,
                stability,
            );

            // <interface name> set(K key, V value);
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                self_name,
                &[OperationId::Operation(Some("set"))],
                [key_arg(), value_arg()],
                &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                    MayBeNull {
                        type_: Identifier(self_name),
                        q_mark: None,
                    },
                )))),
                &None,
                false,
                stability,
            );
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)>
    for weedle::interface::SetlikeInterfaceMember<'src>
{
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        ctx: (&'src str, ApiStability),
    ) -> Result<()> {
        let value_ty = &self.generics.body;
        let value_arg = || Arg {
            attributes: &None,
            name: "value",
            ty: &value_ty.type_,
            optional: false,
            variadic: false,
        };

        let undefined_ret = || ReturnType::Undefined(term!(undefined));

        // readonly attribute unsigned long size;
        record
            .interfaces
            .get_mut(ctx.0)
            .unwrap()
            .attributes
            .push(AttributeInterfaceData {
                definition: &AttributeInterfaceMember {
                    attributes: None,
                    modifier: None,
                    readonly: Some(term!(readonly)),
                    attribute: term!(attribute),
                    type_: AttributedType {
                        attributes: None,
                        type_: Type::Single(SingleType::NonAny(NonAnyType::Integer(MayBeNull {
                            type_: IntegerType::Long(LongType {
                                unsigned: Some(term!(unsigned)),
                                long: term!(long),
                            }),
                            q_mark: None,
                        }))),
                    },
                    identifier: Identifier("size"),
                    semi_colon: term!(;),
                },
                stability: ctx.1,
            });

        // boolean has(V value);
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("has"))],
            [value_arg()],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Boolean(
                MayBeNull {
                    type_: term!(boolean),
                    q_mark: None,
                },
            )))),
            &None,
            false,
            ctx.1,
        );

        // callback SetlikeForEachCallback = undefined (V value);
        // TODO: the signature of the callback is erased, could we keep it?
        let foreach_callback_arg = Arg {
            attributes: &None,
            name: "callback",
            ty: &Type::Single(SingleType::NonAny(NonAnyType::Identifier(MayBeNull {
                type_: Identifier("SetlikeForEachCallback"),
                q_mark: None,
            }))),
            optional: false,
            variadic: false,
        };

        record.callbacks.insert("SetlikeForEachCallback");

        // [Throws] undefined forEach(SetlikeForEachCallback cb);
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("forEach"))],
            [foreach_callback_arg],
            &undefined_ret(),
            &THROWS_ATTR,
            false,
            ctx.1,
        );

        // TODO: iterators could have stronger types by generating specialised interfaces for each
        //       maplike/setlike. Right now, `value` is always `any`.

        // declare the iterator interface
        record.iterators.insert("SetlikeIterator");

        // [NewObject] SetlikeIterator entries();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("entries"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("SetlikeIterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            ctx.1,
        );

        // [NewObject] SetlikeIterator keys();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("keys"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("SetlikeIterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            ctx.1,
        );

        // [NewObject] SetlikeIterator values();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("values"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("SetlikeIterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            ctx.1,
        );

        // add writeable interface if *not* readonly
        if self.readonly.is_none() {
            // undefined clear();
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                ctx.0,
                &[OperationId::Operation(Some("clear"))],
                &[],
                &undefined_ret(),
                &None,
                false,
                ctx.1,
            );

            // boolean delete(V value);
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                ctx.0,
                &[OperationId::Operation(Some("delete"))],
                [value_arg()],
                &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Boolean(
                    MayBeNull {
                        type_: term!(boolean),
                        q_mark: None,
                    },
                )))),
                &None,
                false,
                ctx.1,
            );

            // <interface name> add(V value);
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                ctx.0,
                &[OperationId::Operation(Some("add"))],
                [value_arg()],
                &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                    MayBeNull {
                        type_: Identifier(ctx.0),
                        q_mark: None,
                    },
                )))),
                &None,
                false,
                ctx.1,
            );
        }

        Ok(())
    }
}
impl<'src> FirstPass<'src, (&'src str, ApiStability)>
    for weedle::interface::IterableInterfaceMember<'src>
{
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        ctx: (&'src str, ApiStability),
    ) -> Result<()> {
        record.iterators.insert("Iterator");

        // [NewObject] Iterator entries();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("entries"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("Iterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            ctx.1,
        );

        // [NewObject] Iterator keys();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("keys"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("Iterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            ctx.1,
        );

        // [NewObject] Iterator values();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("values"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("Iterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            ctx.1,
        );

        let undefined_ret = || ReturnType::Undefined(term!(undefined));

        // callback SetlikeForEachCallback = undefined (V value);
        // TODO: the signature of the callback is erased, could we keep it?
        let foreach_callback_arg = Arg {
            attributes: &None,
            name: "callback",
            ty: &Type::Single(SingleType::NonAny(NonAnyType::Identifier(MayBeNull {
                type_: Identifier("IterableForEachCallback"),
                q_mark: None,
            }))),
            optional: false,
            variadic: false,
        };

        record.callbacks.insert("IterableForEachCallback");

        // [Throws] undefined forEach(SetlikeForEachCallback cb);
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("forEach"))],
            [foreach_callback_arg],
            &undefined_ret(),
            &THROWS_ATTR,
            false,
            ctx.1,
        );
        Ok(())
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)>
    for weedle::interface::AsyncIterableInterfaceMember<'src>
{
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        ctx: (&'src str, ApiStability),
    ) -> Result<()> {
        record.async_iterators.insert("AsyncIterator");

        // [NewObject] MapLikeIterator entries();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("entries"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("AsyncIterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            ctx.1,
        );
        // [NewObject] MapLikeIterator keys();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("keys"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("AsyncIterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            ctx.1,
        );
        // [NewObject] MapLikeIterator values();
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            ctx.0,
            &[OperationId::Operation(Some("values"))],
            &[],
            &ReturnType::Type(Type::Single(SingleType::NonAny(NonAnyType::Identifier(
                MayBeNull {
                    type_: Identifier("AsyncIterator"),
                    q_mark: None,
                },
            )))),
            &NEW_OBJECT_ATTR,
            false,
            ctx.1,
        );

        Ok(())
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)>
    for weedle::interface::AttributeInterfaceMember<'src>
{
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        ctx: (&'src str, ApiStability),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        record
            .interfaces
            .get_mut(ctx.0)
            .unwrap()
            .attributes
            .push(AttributeInterfaceData {
                definition: self,
                stability: ctx.1,
            });
        Ok(())
    }
}

impl<'src> FirstPass<'src, ApiStability> for weedle::InterfaceMixinDefinition<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        stability: ApiStability,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        {
            let mixin_data = record.mixins.entry(self.identifier.0).or_default();
            mixin_data.partial = false;
            mixin_data.definition_attributes = self.attributes.as_ref();
            mixin_data.stability = stability;
        }

        for member in &self.members.body {
            member.first_pass(record, (self.identifier.0, stability))?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, ApiStability> for weedle::PartialInterfaceMixinDefinition<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        stability: ApiStability,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        record
            .mixins
            .entry(self.identifier.0)
            .or_insert_with(|| MixinData {
                partial: true,
                stability,
                ..Default::default()
            });

        for member in &self.members.body {
            member.first_pass(record, (self.identifier.0, stability))?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)> for weedle::mixin::MixinMember<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        ctx: (&'src str, ApiStability),
    ) -> Result<()> {
        match self {
            MixinMember::Operation(op) => op.first_pass(record, ctx),
            MixinMember::Attribute(a) => a.first_pass(record, ctx),
            MixinMember::Const(a) => {
                if util::is_chrome_only(&a.attributes) {
                    return Ok(());
                }
                record.mixins.get_mut(ctx.0).unwrap().consts.push(a);
                Ok(())
            }
            MixinMember::Stringifier(_) => {
                log::warn!("Unsupported WebIDL stringifier mixin member: {self:?}");
                Ok(())
            }
        }
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)>
    for weedle::mixin::OperationMixinMember<'src>
{
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        ctx: (&'src str, ApiStability),
    ) -> Result<()> {
        if self.stringifier.is_some() {
            log::warn!("Unsupported webidl stringifier: {self:?}");
            return Ok(());
        }

        first_pass_operation(
            record,
            FirstPassOperationType::Mixin,
            ctx.0,
            &[OperationId::Operation(self.identifier.map(|s| s.0))],
            &self.args.body.list,
            &self.return_type,
            &self.attributes,
            false,
            ctx.1,
        );
        Ok(())
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)>
    for weedle::mixin::AttributeMixinMember<'src>
{
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        ctx: (&'src str, ApiStability),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }
        record
            .mixins
            .get_mut(ctx.0)
            .unwrap()
            .attributes
            .push(AttributeMixinData {
                definition: self,
                stability: ctx.1,
            });
        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::TypedefDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if record
            .typedefs
            .insert(self.identifier.0, &self.type_.type_)
            .is_some()
        {
            log::info!(
                "Encountered multiple typedef declarations: {}",
                self.identifier.0
            );
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, ApiStability> for weedle::NamespaceDefinition<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        stability: ApiStability,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        let namespace = record.namespaces.entry(self.identifier.0).or_default();
        namespace.stability = stability;

        for member in &self.members.body {
            member.first_pass(record, (self.identifier.0, stability))?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, ApiStability> for weedle::PartialNamespaceDefinition<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        stability: ApiStability,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        for member in &self.members.body {
            member.first_pass(record, (self.identifier.0, stability))?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)> for weedle::namespace::NamespaceMember<'src> {
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        ctx: (&'src str, ApiStability),
    ) -> Result<()> {
        match self {
            weedle::namespace::NamespaceMember::Const(const_) => {
                record
                    .namespaces
                    .get_mut(ctx.0)
                    .unwrap()
                    .consts
                    .push(ConstNamespaceData {
                        definition: const_,
                        stability: ctx.1,
                    });
                Ok(())
            }
            weedle::namespace::NamespaceMember::Operation(op) => op.first_pass(record, ctx),
            _ => Ok(()),
        }
    }
}

impl<'src> FirstPass<'src, (&'src str, ApiStability)>
    for weedle::namespace::OperationNamespaceMember<'src>
{
    fn first_pass(
        &'src self,
        record: &mut FirstPassRecord<'src>,
        (self_name, stability): (&'src str, ApiStability),
    ) -> Result<()> {
        first_pass_operation(
            record,
            FirstPassOperationType::Namespace,
            self_name,
            &[OperationId::Operation(self.identifier.map(|s| s.0))],
            &self.args.body.list,
            &self.return_type,
            &self.attributes,
            true,
            stability,
        );
        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::CallbackDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, _: ()) -> Result<()> {
        record.callbacks.insert(self.identifier.0);
        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::CallbackInterfaceDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, _: ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }
        if self.inheritance.is_some() {
            log::warn!(
                "skipping callback interface with inheritance: {}",
                self.identifier.0
            );
            return Ok(());
        }
        let data = CallbackInterfaceData {
            definition: self,
            single_function: self.members.body.len() == 1,
        };
        record.callback_interfaces.insert(self.identifier.0, data);
        Ok(())
    }
}

impl<'a> FirstPassRecord<'a> {
    pub fn all_superclasses<'me>(&'me self, interface: &str) -> impl Iterator<Item = String> + 'me {
        let mut set = BTreeSet::new();
        let mut list = Vec::new();
        self.fill_superclasses(interface, &mut set, &mut list);
        list.into_iter()
    }

    fn fill_superclasses(
        &self,
        interface: &str,
        set: &mut BTreeSet<&'a str>,
        list: &mut Vec<String>,
    ) {
        let data = match self.interfaces.get(interface) {
            Some(data) => data,
            None => return,
        };
        let superclass = match &data.superclass {
            Some(class) => class,
            None => return,
        };
        if self.interfaces.contains_key(superclass) && set.insert(superclass) {
            list.push(camel_case_ident(superclass));
            self.fill_superclasses(superclass, set, list);
        }
    }

    pub fn all_mixins<'me>(
        &'me self,
        interface: &str,
    ) -> impl Iterator<Item = &'me MixinData<'a>> + 'me {
        let mut set = Vec::new();
        self.fill_mixins(interface, &mut set);
        set.into_iter()
    }

    fn fill_mixins<'me>(&'me self, mixin_name: &str, list: &mut Vec<&'me MixinData<'a>>) {
        if let Some(mixin_data) = self.mixins.get(mixin_name) {
            list.push(mixin_data);
        }
        if let Some(mixin_names) = self.includes.get(mixin_name) {
            for mixin_name in mixin_names {
                self.fill_mixins(mixin_name, list);
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IgnoreTraits<T>(pub T);

impl<T> PartialEq for IgnoreTraits<T> {
    fn eq(&self, _other: &IgnoreTraits<T>) -> bool {
        true
    }
}

impl<T> Eq for IgnoreTraits<T> {}

impl<T> PartialOrd for IgnoreTraits<T> {
    fn partial_cmp(&self, _other: &IgnoreTraits<T>) -> Option<Ordering> {
        Some(self.cmp(_other))
    }
}

impl<T> Ord for IgnoreTraits<T> {
    fn cmp(&self, _other: &IgnoreTraits<T>) -> Ordering {
        Ordering::Equal
    }
}
