// jshint esversion:9
"use strict";
import { createInterface } from "readline";

const rl = createInterface({
    input: process.stdin,
    output: process.stdout
});

function ask(prompt) {
    return new Promise(resolve => {
        rl.question(prompt, resolve);
    });
}

const config = {
    startingAt: 1
};

void async function () {
    while (true) await getPattern();
}();

async function getPattern() {
    const pattern = (await ask("Pattern: ")).split(",").map(parseNum);
    console.log(polynomial(pattern));
}

function parseNum(str) {
    str = str.trim();
    if (/(?=^\d*\.?\d*$)(?=.*\d)/.test(str)) return parseFloat(str); // Is a whole or decimal number
    else if (/^\d+\/\d+$/.test(str)) { // A fraction
        const fraction = str.split("/");
        return parseNum(fraction[0]) / parseNum(fraction[1]);
    }
    else if (/^(?:\d*\s+)?\d+\/\d+$/.test(str)) { // A mixed number
        const split = str.split(" ");
        return parseInt(split[0]) + parseNum(split[1]);
    }
    else throw new Error(str + " is not a number"); // needs to be handled
}

function polynomial(values) {
    if (values.length < 2) {
        // Cannot find pattern with 0 or 1 numbers
    }
    if (values.areAllEqual()) return { coefficient: values[0], exponent: 0 };
    const { degree: deg, nthDiff: diff } = degree(values);
    //console.log(degree(values));
    //console.log(values, deg, diff);
    if (isNaN(deg)) return null;
    const coefficient = diff / Math.factorial(deg);
    let arr =  [{ coefficient, exponent: deg }];
    if (diff !== 0) arr = arr.concat(polynomial(values.map((e, i) => e / (coefficient * (i + config.startingAt) ** deg))));
    return arr;
    // I know have the degree and coefficient for the term with the highest degree
    // Repeat this recursively (solve the quadratic withoutHighestDegree) and add coefficient*x^degree
}
function degree(values) { // returns degree of a polynomial function that produces these values or NaN
    if (values.length < 3) return {degree: NaN, nthDiff: NaN};
    const diffs = [];
    for (let i = 0; i < values.length - 1; i++) diffs.push(values[i + 1] - values[i]);
    if (diffs.areAllEqual()) return { degree: 1, nthDiff: diffs[0] };
    else {
        const rec = degree(diffs);
        rec.degree++;
        return rec;
    }
    /*for (let i = 0; i < diffs.length - 1; i++) {
        if (diffs[i] !== diffs[i + 1]) {
            
        }
    }*/
    //console.log(diffs[0]);
    
}

Math.factorial = n => {
    if (n === 0 || n === 1) return 1;
    else if (n > 1) return n * Math.factorial(n - 1);
};
Array.prototype.areAllEqual = function() {
    for (let i = 0; i < this.length - 1; i++) {
        if (this[i] !== this[i + 1]) return false;
    }
    return true;
};