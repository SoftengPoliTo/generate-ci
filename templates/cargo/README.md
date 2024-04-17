# {{ name }}

[![Actions Status][actions badge]][actions]
[![CodeCov][codecov badge]][codecov]
[![Wcc][wcc badge]][wcc]
[![LICENSE][license badge]][license]

{{ name }}'s description

**Note that**: the created Github Actions workflow uses a static code analysis tool, called `weighted-code-coverage`, which produces an `html` report that is hosted on [Github Pages](https://docs.github.com/en/pages). However, to make it work, you need to set the select box to **GitHub Actions** in the **GitHub Pages** panel contained in **Settings**, as described [here](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site#publishing-with-a-custom-github-actions-workflow).

# Acknowledgements

<!-- Links -->
[actions]: https://github.com/<your-account>/{{ name }}/actions
[codecov]: https://codecov.io/gh/<your-account>/{{ name }}
[wcc]: https://<your-account>.github.io/{{ name }}
[license]: LICENSES/{{ license_id }}.txt

<!-- Badges -->
[actions badge]: https://github.com/<your-account>/{{ name }}/workflows/{{ name }}/badge.svg
[codecov badge]: https://codecov.io/gh/<your-account>/{{ name }}/branch/master/graph/badge.svg
[wcc badge]: .github/badges/wcc.svg
[license badge]: https://img.shields.io/badge/license-{{ license_id }}-blue.svg
