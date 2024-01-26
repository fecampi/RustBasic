const native = require('.');

const result1 = native.hello();
console.log(result1); // "Hello Node"

const result2 = native.addNumbers(2, 3);
console.log(result2); // 5

const result3 = native.customGreet("Alice", 30, 1.75);
console.log(result3); // "Hello, Alice! You are 30 years old and 1.75 meters tall."

const result4 = native.calculateHypotenuse(3, 6);
console.log(result4); // 5
