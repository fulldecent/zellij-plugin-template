# Project name

[![Lint](https://github.com/fulldecent/project-template/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/fulldecent/project-template/actions/workflows/lint.yml)

> [!TIP]
> Replace this heading with the project name and status badge, and replace the rest of this section with what the project does.

This is an opinionated template for every project, unless a more specific template applies, that provides:

- An MIT license ([LICENSE](LICENSE))
- [EditorConfig](.editorconfig) with modern defaults.
- A [.gitignore](.gitignore) with modern defaults.
- GitHub Actions workflow to [lint file formatting](.github/workflows/lint.yml).
- Markdown is checked by markdownlint; Prettier skips it via `.prettierignore`.

## More specific templates

Use a more specific template if it applies. These all conform to this project template and provide additional features:

- [Node.js template](https://github.com/fulldecent/node.js-template): Node.js module (e.g. on NPM) or application
- [GitHub Pages template](https://github.com/fulldecent/github-pages-template): collaboratively edited HTML websites
- [Swift 6 module template](https://github.com/fulldecent/swift6-module-template): reusable Swift 6 module (e.g. with Swift Package Manager)
- [Solidity template](https://github.com/fulldecent/solidity-template): Solidity contracts (technology preview)
- [Moodle plugin template](https://github.com/fulldecent/moodle-local_plugin_template): Moodle plugin (work in progress)
- [Podcast template](https://github.com/fulldecent/podcast-template): podcast on your own domain

## Maintenance and dependency updates

Do this every month or so and please send a PR here if you see updates available:

1. Identify external Actions in [.github/workflows](./.github/workflows) scripts and look for available new versions. Review and then update to the new version if it is safe. GitHub-supported Actions (i.e. under the actions/ organization) may require only cursory review.

## References

1. We include a GitHub Action for validating file format but do not provide instructions to automate that formatting. Your own project may wish to do that if people developing your project are comfortable using the command line and installing packages.
1. We use title case for titles and proper nouns; not for headings and things. This includes our README above as well as our workflow rules and other configuration files. If you have a different policy, then please implement it throughout.
1. This project is built based on [best practices documented in project-template](https://github.com/fulldecent/project-template), release 1.0.0.
