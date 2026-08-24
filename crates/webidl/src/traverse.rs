use syn::{Ident, Type};

pub trait TraverseType {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident);
}

impl TraverseType for Type {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        match self {
            Type::Array(x) => x.traverse_type(f),
            Type::FnPtr(x) => x.traverse_type(f),
            Type::Group(x) => x.traverse_type(f),
            Type::Paren(x) => x.traverse_type(f),
            Type::Path(x) => x.traverse_type(f),
            Type::Ptr(x) => x.traverse_type(f),
            Type::Reference(x) => x.traverse_type(f),
            Type::Slice(x) => x.traverse_type(f),
            Type::Tuple(x) => x.traverse_type(f),
            _ => {}
        }
    }
}

impl TraverseType for syn::TypeArray {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        self.elem.traverse_type(f);
    }
}

impl TraverseType for syn::TypeFnPtr {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        for input in self.inputs.iter() {
            input.ty.traverse_type(f);
        }

        self.output.traverse_type(f);
    }
}

impl TraverseType for syn::ReturnType {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        match self {
            Self::Default => {}
            Self::Type(_, ty) => {
                ty.traverse_type(f);
            }
        }
    }
}

impl TraverseType for syn::TypeGroup {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        self.elem.traverse_type(f);
    }
}

impl TraverseType for syn::TypeParen {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        self.elem.traverse_type(f);
    }
}

impl TraverseType for syn::TypePath {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        if let Some(qself) = self.qself.as_ref() {
            qself.traverse_type(f);
        }

        if let Some(last) = self.path.segments.last() {
            f(&last.ident);
            last.arguments.traverse_type(f);
        }
    }
}

impl TraverseType for syn::QSelf {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        self.ty.traverse_type(f);
    }
}

impl TraverseType for syn::PathArguments {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        match self {
            Self::None => {}
            Self::AngleBracketed(x) => x.traverse_type(f),
            Self::Parenthesized(x) => x.traverse_type(f),
        }
    }
}

impl TraverseType for syn::AngleBracketedGenericArguments {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        for ty in self.args.iter() {
            ty.traverse_type(f);
        }
    }
}

impl TraverseType for syn::GenericArgument {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        match self {
            Self::Type(x) => x.traverse_type(f),
            Self::AssocType(x) => x.traverse_type(f),
            _ => {}
        }
    }
}

impl TraverseType for syn::AssocType {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        self.ty.traverse_type(f);
    }
}

impl TraverseType for syn::ParenthesizedGenericArguments {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        for arg in self.inputs.iter() {
            arg.ty.traverse_type(f);
        }

        self.output.traverse_type(f);
    }
}

impl TraverseType for syn::TypePtr {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        self.elem.traverse_type(f);
    }
}

impl TraverseType for syn::TypeReference {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        self.elem.traverse_type(f);
    }
}

impl TraverseType for syn::TypeTuple {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        for ty in self.elems.iter() {
            ty.traverse_type(f);
        }
    }
}

impl TraverseType for syn::TypeSlice {
    fn traverse_type<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        self.elem.traverse_type(f);
    }
}
