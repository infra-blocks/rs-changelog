//! These tests are the examples taken from the [specification](https://spec.commonmark.org/0.31.2/).
use ast::{Document, ToHtml};

// TODO: put all the tests.

macro_rules! test {
    ($name:ident, $markdown:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let document = Document::parse($markdown);
            assert_eq!(document.to_html(), $expected);
        }
    };
}

mod atx_heading {
    use super::*;

    test!(
        example_62,
        r"# foo
## foo
### foo
#### foo
##### foo
###### foo",
        r"<h1>foo</h1>
<h2>foo</h2>
<h3>foo</h3>
<h4>foo</h4>
<h5>foo</h5>
<h6>foo</h6>"
    );
}

mod blank_line {
    use super::*;

    test!(
        example_227,
        r"

aaa


# aaa

  ",
        r"<p>aaa</p>
<h1>aaa</h1>"
    );
}

mod fenced_code {
    use super::*;

    test!(
        example_119,
        r"```
<
 >
```",
        r"<pre><code>&lt;
 &gt;
</code></pre>"
    );
    test!(
        example_120,
        r"~~~
<
 >
~~~",
        r"<pre><code>&lt;
 &gt;
</code></pre>"
    );
    test!(
        example_121,
        r"``
foo
``",
        r"<p><code>foo</code></p>"
    );
    test!(
        example_122,
        r"```
aaa
~~~
```",
        r"<pre><code>aaa
~~~
</code></pre>"
    );
    test!(
        example_123,
        r"~~~
aaa
```
~~~",
        r"<pre><code>aaa
```
</code></pre>"
    );
    test!(
        example_124,
        r"````
aaa
```
``````",
        r"<pre><code>aaa
```
</code></pre>"
    );
    test!(
        example_125,
        r"~~~~
aaa
~~~
~~~~",
        r"<pre><code>aaa
~~~
</code></pre>"
    );
    test!(example_126, r"```", r"<pre><code></code></pre>");
    test!(
        example_127,
        r"`````

```
aaa",
        r"<pre><code>
```
aaa
</code></pre>"
    );
}

mod indented_code {
    use super::*;

    test!(
        example_107,
        r"    a simple
      indented code block
",
        r"<pre><code>a simple
  indented code block
</code></pre>"
    );

    test!(
        example_111,
        r"    chunk1

    chunk2



    chunk3
",
        r"<pre><code>chunk1

chunk2



chunk3
</code></pre>"
    );
}

mod thematic_break {
    use super::*;

    test!(
        example_43,
        r"***
---
___",
        r"<hr />
<hr />
<hr />"
    );
}
