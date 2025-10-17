// Create a custom thenable that resolves to a Number
exports.createNumericThenable = function(value) {
  return {
    then: function(onFulfilled) {
      // Simulate async resolution
      return Promise.resolve(value).then(v => onFulfilled(v));
    }
  };
};

// A function that accepts anything that promises a Number
// This demonstrates that the generic Promising trait works correctly
exports.processNumericPromising = async function(promising) {
  // Use Promise.resolve to handle both Promises and thenables
  const num = await Promise.resolve(promising);
  return `Number: ${num}`;
};
