# swc-plugin-react-remove-properties

SWC plugin for removing React properties.
You can use this plugin to remove those properties you don't want in final output,
such as test IDs `data-test-...`, etc.

## Installation

```
npm i -D swc-plugin-react-remove-properties
```

## Configuration

You can configure your `.swcrc` file.

### Minimum Config

```json
{
    "jsc": {
        "parser": {
            "syntax": "ecmascript",
            "jsx": true
        },
        "experimental": {
            "plugins": [["swc-plugin-react-remove-properties", {}]]
        }
    }
}
```

By default, it will remove the properties that start with `data-test`.

### Configure Your Patterns

```json
{
    "jsc": {
        "parser": {
            "syntax": "ecmascript",
            "jsx": true
        },
        "experimental": {
            "plugins": [
                [
                    "swc-plugin-react-remove-properties",
                    {
                        "properties": ["/^data-test/", "unwanted"]
                    }
                ]
            ]
        }
    }
}
```

As the example above, you can add your own patterns in `properties` config item.
You can use strings or regexes. If you use regex, please put it inside the slash (`/`);
otherwise, it will be treated as a normal string which requires exact match.
You can put regex flags after the suffix slash.

Please note that once you configured with your patterns,
they will override the default value (which is `["/^data-test/"]`).
You may need to add it again if you want.

## License

MIT License

Copyright (c) 2022-present Pig Fang
