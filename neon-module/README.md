# neon-module

Rust bindings for node.js. Based on the tutorial [here](https://neon-bindings.com/docs/intro)

Install:
```bash
$ sudo npm install --global neon-cli
```

Build:
```bash
$ neon build --release

```
Run:
```js
$ node
Welcome to Node.js v12.14.0.
Type ".help" for more information.
> neon=require("./native");
{ hello: [Function], objop: [Function], thread_count: [Function] }
> neon.hello()
{
  id: 'node',
  values: [
     2,  3,  5,  7,
    11, 13, 17, 19
  ],
  x: 4
}
> neon.thread_count()
4
> data={id:"me", values:[1,2,3], x:1.0}
{ id: 'me', values: [ 1, 2, 3 ], x: 1 }
> data=neon.objop(data)
{ id: 'me_', values: [ 1, 2, 3, 9, 8, 7 ], x: 1.5 }
>
```