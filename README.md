# rs-changelog
[![Build](https://github.com/infra-blocks/rs-changelog/actions/workflows/build.yml/badge.svg)](https://github.com/infra-blocks/rs-changelog/actions/workflows/build.yml)
[![Release](https://github.com/infra-blocks/rs-changelog/actions/workflows/release.yml/badge.svg)](https://github.com/infra-blocks/rs-changelog/actions/workflows/release.yml)
[![Update From Template](https://github.com/infra-blocks/rs-changelog/actions/workflows/update-from-template.yml/badge.svg)](https://github.com/infra-blocks/rs-changelog/actions/workflows/update-from-template.yml)
[![codecov](https://codecov.io/gh/infra-blocks/rs-changelog/graph/badge.svg?token=XFP3KC9OBA)](https://codecov.io/gh/infra-blocks/rs-changelog)

## Possible interesting implementations

### Markdown parsing

- Each segment could carry information after each parsing attempt. For example, the original segment could be turned into either AtxHeadingSegment(segment) or NotAtxHeadingSegment(segment) after the
parsing attempt. This, however, means that "segment" needs to become a trait to simplify the code hehe. One advantage is that an overload can be made for NotAtxHeadingSegment, for example, where
the state machine immediately returns the input without attempting to parse. I'll keep an eye out for where this sort of information could be reused. It becomes interesting when there is some
cross talk possible. For example, one parsing could be optimized if it knows that the input is NonBlankLineSegment().

