import { current_allocation, MyStruct } from "./pkg/explicit_resource_management.js";

const initialAllocation = current_allocation();
let referrent = {};
console.log('Before scope: ', initialAllocation);

{
    using foo = new MyStruct("Rust");
    // force foo to be treated as live by implicit memory management (FinalizationRegistry/GC)
    // by retaining a reference that outlives the scope block (for the purposes of proving
    // Symbol.dispose is called on scope exit).
    referrent['foo'] = foo;
    console.log('After construction, but before scope exit: ', current_allocation());
}
const afterDisposeAllocation = current_allocation();
console.log('After scope exit: ', afterDisposeAllocation);
console.log(referrent);