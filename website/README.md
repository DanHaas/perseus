# Website

This directory contains the website for Perseus, which is hosted at <https://framesurge.sh/perseus>!

## Comparisons

The website includes a [comparisons page](https://framesurge.sh/perseus/en-US/comparisons), which compares Perseus to a number of other frameworks. Of course, there are _a lot_ of frameworks out there, so we highly encourage contributions to this! It's designed to be quite easy to contribute to, just add a new file called `website/comparisons/framework.json` (substituting `framework` for the name of the framework) and fill in the following framework details:

-   `name`: `String`,
-   `supports_ssg`: `"full"`/`"partial"`/`"none"`,
-   `supports_ssr`: `"full"`/`"partial"`/`"none"`,
-   `supports_ssr_ssg_same_page`: `"full"`/`"partial"`/`"none"`,
-   `supports_i18n`: `"full"`/`"partial"`/`"none"`,
-   `supports_incremental`: `"full"`/`"partial"`/`"none"`,
-   `supports_revalidation`: `"full"`/`"partial"`/`"none"`,
-   `inbuilt_cli`: `"full"`/`"partial"`/`"none"`,
-   `inbuilt_routing`: `"full"`/`"partial"`/`"none"`,
-   `supports_shell`: `"full"`/`"partial"`/`"none"`,
-   `supports_deployment`: `"full"`/`"partial"`/`"none"`,
-   `supports_exporting`: `"full"`/`"partial"`/`"none"`,
-   `language`: `String`,
-   `homepage_lighthouse_desktop`: `u8`,
-   `homepage_lighthouse_mobile`: `u8`

### Lighthouse Scores

For consistency, we generate all Lighthouse scores through [PageSpeed Insights](https://developers.google.com/speed/pagespeed/insights). As this metric can vary slightly between machines and runs, it's advised to run it more than once and take an average (rounding up). Maintainers will check these themselves, and if there's any major discrepancy (>5 points), you may be asked to provide a screenshot from your system. Maintainers reserve the right to determine the final verdict on which score to use in the event of a conflict.
