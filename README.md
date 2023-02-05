# Wordsmith

Wordsmith is a `cli` that helps you to write ebooks using Markdown.

## Features

- [x] Custom cover photo
- [x] Code syntax highlight
- [x] Custom themes
- [x] Custom components/tags
- [x] Custom fonts
- [ ] Table of contents
- [ ] Header/Footer

> TODO: headless chrome does not use author meta tags;
> find a way to update the pdf info

## Installation

> TODO
> brew, curl, ...

## Usage

> TODO: describe all functionalities

```sh
wordsmith init new-project
```

```sh
wordsmith build light
```

## Special tags

```blade
@info something @endinfo
```

```blade
@danger something @enddanger
```

```blade
@warn something @endwarn
```

```blade
@quote something @endquote
```

```blade
![My image](@assets_path/images/image.png)
```

```blade
@break
```

## Credits

This project was inspired by and is similar to [ibis](https://github.com/themsaid/ibis/), but has different features and cli commands.

Images: [image.png](https://www.iconfinder.com/search?q=ebook&style=solid&price=free), [cover.jpg](https://unsplash.com/@anniespratt).

## Author

[Eduardo Stuart](https://s.tuart.dev)

## License

The MIT License (MIT). Please see [License File](./LICENSE.md) for more information.
