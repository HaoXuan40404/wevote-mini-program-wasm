# 小程序wasm

 wasm-pack build --release --target web
 wasm-pack build --release --target no-modules
 wasm-pack build --release --target nodejs

 ## 小程序修改

- 注释以下代码

 ```bash
 async function init(input) {
    // if (typeof input === 'undefined') {
    //     input = new URL('ffi_wasm_wevote_bg.wasm', import.meta.url);
    // }
    const imports = getImports();

    // if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
    //     input = fetch(input);
    // }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}
```

- webAssembly替换为WXWebAssembly

