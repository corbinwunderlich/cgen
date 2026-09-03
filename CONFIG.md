# Settings

- [1. Property `Settings > inputs`](#inputs)
  - [1.1. Property `Settings > inputs > clang`](#inputs_clang)
    - [1.1.1. Property `Settings > inputs > clang > extensions`](#inputs_clang_extensions)
      - [1.1.1.1. Settings > inputs > clang > extensions > extensions items](#inputs_clang_extensions_items)
- [2. Property `Settings > outputs`](#outputs)
  - [2.1. Property `Settings > outputs > all`](#outputs_all)
    - [2.1.1. Property `Settings > outputs > all > watermark`](#outputs_all_watermark)
  - [2.2. Property `Settings > outputs > c_header`](#outputs_c_header)
    - [2.2.1. Property `Settings > outputs > c_header > extension`](#outputs_c_header_extension)
  - [2.3. Property `Settings > outputs > enable`](#outputs_enable)
    - [2.3.1. Settings > outputs > enable > OutputKind](#outputs_enable_items)

**Title:** Settings

|                           |                  |
| ------------------------- | ---------------- |
| **Type**                  | `object`         |
| **Required**              | No               |
| **Additional properties** | Any type allowed |

| Property               | Pattern | Type   | Deprecated | Definition               | Title/Description |
| ---------------------- | ------- | ------ | ---------- | ------------------------ | ----------------- |
| - [inputs](#inputs )   | No      | object | No         | In #/$defs/InputsConfig  | -                 |
| - [outputs](#outputs ) | No      | object | No         | In #/$defs/OutputsConfig | -                 |

## <a name="inputs"></a>1. Property `Settings > inputs`

|                           |                      |
| ------------------------- | -------------------- |
| **Type**                  | `object`             |
| **Required**              | No                   |
| **Additional properties** | Any type allowed     |
| **Defined in**            | #/$defs/InputsConfig |

| Property                  | Pattern | Type   | Deprecated | Definition             | Title/Description |
| ------------------------- | ------- | ------ | ---------- | ---------------------- | ----------------- |
| - [clang](#inputs_clang ) | No      | object | No         | In #/$defs/ClangConfig | -                 |

### <a name="inputs_clang"></a>1.1. Property `Settings > inputs > clang`

|                           |                     |
| ------------------------- | ------------------- |
| **Type**                  | `object`            |
| **Required**              | No                  |
| **Additional properties** | Any type allowed    |
| **Defined in**            | #/$defs/ClangConfig |

| Property                                  | Pattern | Type            | Deprecated | Definition | Title/Description                        |
| ----------------------------------------- | ------- | --------------- | ---------- | ---------- | ---------------------------------------- |
| - [extensions](#inputs_clang_extensions ) | No      | array of string | No         | -          | The extensions which are parsed by Clang |

#### <a name="inputs_clang_extensions"></a>1.1.1. Property `Settings > inputs > clang > extensions`

|              |                                               |
| ------------ | --------------------------------------------- |
| **Type**     | `array of string`                             |
| **Required** | No                                            |
| **Default**  | `["c", "cpp", "cc", "cxx", "c++", "m", "mm"]` |

**Description:** The extensions which are parsed by Clang

|                      | Array restrictions |
| -------------------- | ------------------ |
| **Min items**        | N/A                |
| **Max items**        | N/A                |
| **Items unicity**    | False              |
| **Additional items** | False              |
| **Tuple validation** | See below          |

| Each item of this array must be                    | Description |
| -------------------------------------------------- | ----------- |
| [extensions items](#inputs_clang_extensions_items) | -           |

##### <a name="inputs_clang_extensions_items"></a>1.1.1.1. Settings > inputs > clang > extensions > extensions items

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | No       |

## <a name="outputs"></a>2. Property `Settings > outputs`

|                           |                       |
| ------------------------- | --------------------- |
| **Type**                  | `object`              |
| **Required**              | No                    |
| **Additional properties** | Any type allowed      |
| **Defined in**            | #/$defs/OutputsConfig |

| Property                         | Pattern | Type   | Deprecated | Definition                  | Title/Description     |
| -------------------------------- | ------- | ------ | ---------- | --------------------------- | --------------------- |
| - [all](#outputs_all )           | No      | object | No         | In #/$defs/AllOutputsConfig | -                     |
| - [c_header](#outputs_c_header ) | No      | object | No         | In #/$defs/CHeaderConfig    | -                     |
| - [enable](#outputs_enable )     | No      | array  | No         | -                           | The outputs to enable |

### <a name="outputs_all"></a>2.1. Property `Settings > outputs > all`

|                           |                          |
| ------------------------- | ------------------------ |
| **Type**                  | `object`                 |
| **Required**              | No                       |
| **Additional properties** | Any type allowed         |
| **Defined in**            | #/$defs/AllOutputsConfig |

| Property                               | Pattern | Type    | Deprecated | Definition | Title/Description                                           |
| -------------------------------------- | ------- | ------- | ---------- | ---------- | ----------------------------------------------------------- |
| - [watermark](#outputs_all_watermark ) | No      | boolean | No         | -          | Adds a watermark displaying cgen version to generated files |

#### <a name="outputs_all_watermark"></a>2.1.1. Property `Settings > outputs > all > watermark`

|              |           |
| ------------ | --------- |
| **Type**     | `boolean` |
| **Required** | No        |
| **Default**  | `true`    |

**Description:** Adds a watermark displaying cgen version to generated files

### <a name="outputs_c_header"></a>2.2. Property `Settings > outputs > c_header`

|                           |                       |
| ------------------------- | --------------------- |
| **Type**                  | `object`              |
| **Required**              | No                    |
| **Additional properties** | Any type allowed      |
| **Defined in**            | #/$defs/CHeaderConfig |

| Property                                    | Pattern | Type   | Deprecated | Definition | Title/Description                         |
| ------------------------------------------- | ------- | ------ | ---------- | ---------- | ----------------------------------------- |
| - [extension](#outputs_c_header_extension ) | No      | string | No         | -          | The output extension for the header files |

#### <a name="outputs_c_header_extension"></a>2.2.1. Property `Settings > outputs > c_header > extension`

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | No       |
| **Default**  | `"h"`    |

**Description:** The output extension for the header files

**Examples:**

```json
"hpp"
```

```json
"h"
```

### <a name="outputs_enable"></a>2.3. Property `Settings > outputs > enable`

|              |         |
| ------------ | ------- |
| **Type**     | `array` |
| **Required** | No      |

**Description:** The outputs to enable

|                      | Array restrictions |
| -------------------- | ------------------ |
| **Min items**        | N/A                |
| **Max items**        | N/A                |
| **Items unicity**    | False              |
| **Additional items** | False              |
| **Tuple validation** | See below          |

| Each item of this array must be     | Description |
| ----------------------------------- | ----------- |
| [OutputKind](#outputs_enable_items) | -           |

#### <a name="outputs_enable_items"></a>2.3.1. Settings > outputs > enable > OutputKind

|                |                    |
| -------------- | ------------------ |
| **Type**       | `enum (of string)` |
| **Required**   | No                 |
| **Defined in** | #/$defs/OutputKind |

Must be one of:
* "c_header"

----------------------------------------------------------------------------------------------------------------------------
