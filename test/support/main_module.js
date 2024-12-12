globalThis.foo = "this is a main module";

await new Promise((fulfill) => setTimeout(fulfill, 1));

globalThis.bar = "this is from another tick";
