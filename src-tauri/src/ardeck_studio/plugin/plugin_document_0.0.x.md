

# Manifest
```js
type Manifest = {
    name: string;               // Sample plugin
    version: string;            // x.y.z
    id: string;                 // com.akurakuu.sample_plugin
    description?: string;       // this is a sample plugin.
    author: string;             // akurakuu
    main: string;               // main.exe
}
```

# Action data
```js
type ActionData = {
    switch_type: SwitchType;
    id: number; // [Rust]: u8
    state: number; // [Rust]: u16
    raw_data: number[]; // [Rust]: Vec<u8>
    timestamp: BigInt; // [Rust]: i64 (timestamp milli)
}

enum SwitchType = {
    Unknown = -1,
    Digital = 0,
    Analog = 1,
}
```
