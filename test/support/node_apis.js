import querystring from 'node:querystring';

globalThis.foo = querystring.escape("this is converted using Node APIs");
