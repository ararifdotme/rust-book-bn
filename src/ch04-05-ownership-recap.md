## ownership-এর পুনরালোচনা {#ownership-recap}

এই অধ্যায়ে ownership, borrowing আর slice-এর মতো অনেক নতুন ধারণার সঙ্গে পরিচয় করানো হয়েছে।
systems programming-এর সঙ্গে পরিচিত না হলে এই অধ্যায়ে আপনি memory allocation, stack বনাম heap, pointer আর undefined behavior-এর মতো নতুন ধারণার সঙ্গেও পরিচিত হয়েছেন। Rust-এর বাকি অংশে যাওয়ার আগে চলুন একটু থেমে দম নিই। এই অধ্যায়ের মূল ধারণাগুলো আমরা ঝালিয়ে নেব এবং অনুশীলন করব।

### ownership বনাম garbage collection {#ownership-versus-garbage-collection}

ownership-কে প্রেক্ষাপটে রাখতে আমাদের **garbage collection** (অব্যবহৃত মেমোরি আপনা-আপনি মুক্ত করার ব্যবস্থা) নিয়ে কথা বলা উচিত।
বেশিরভাগ প্রোগ্রামিং ভাষা মেমোরি সামলাতে একটি garbage collector ব্যবহার করে, যেমন Python, Javascript, Java আর Go। garbage collector runtime-এ চলমান প্রোগ্রামের পাশাপাশি কাজ করে (অন্তত একটি tracing collector তা-ই করে)। collector মেমোরি ঘেঁটে এমন ডেটা খোঁজে, যা আর ব্যবহার হচ্ছে না &mdash; অর্থাৎ চলমান প্রোগ্রাম কোনো function-এর local variable (শুধু একটি function-এর ভেতরে থাকা variable) থেকে আর সেই ডেটায় পৌঁছাতে পারে না। তারপর collector পরে ব্যবহারের জন্য অব্যবহৃত মেমোরি deallocate (বরাদ্দ করা মেমোরি ফেরত দেওয়া) করে।

garbage collector-এর প্রধান সুবিধা হলো, এটি C বা C++-এ যেমন ঘটতে পারে, তেমন undefined behavior (অনির্ধারিত আচরণ, যার ফল ভাষা নির্দিষ্ট করে না; যেমন free করা মেমোরি ব্যবহার) এড়ায়। garbage collection থাকলে undefined behavior যাচাইয়ের জন্য Rust-এর মতো জটিল type system-এরও দরকার হয় না। তবে garbage collection-এর কয়েকটি অসুবিধা আছে। একটি স্পষ্ট অসুবিধা হলো পারফরম্যান্স, কারণ garbage collection-এর ফলে হয় ঘনঘন ছোট বাড়তি খরচ হয় (reference counting-এর জন্য, যেমন Python আর Swift-এ), নয়তো মাঝে মাঝে বড় বাড়তি খরচ হয় (tracing-এর জন্য, যেমন garbage collection ব্যবহার করা বাকি সব ভাষায়)। 

তবে আরেকটি কম স্পষ্ট অসুবিধা হলো **garbage collection-এর আচরণ আগে থেকে বোঝা কঠিন হতে পারে**। বিষয়টি দেখাতে ধরুন আমরা একটি `Document` type implement করছি, যা শব্দের একটি পরিবর্তনযোগ্য তালিকা প্রকাশ করে। Python-এর মতো garbage collection-যুক্ত ভাষায় আমরা `Document` এভাবে implement করতে পারি:

```python
class Document:     
    def __init__(self, words: List[str]):
        """Create a new document"""
        self.words = words

    def add_word(self, word: str):
        """Add a word to the document"""
        self.words.append(word)
        
    def get_words(self) -> List[str]:  
        """Get a list of all the words in the document"""
        return self.words
```

এই `Document` class ব্যবহারের একটি উপায় এখানে দেওয়া হলো, যা একটি document `d` তৈরি করে, সেটিকে একটি নতুন document `d2`-তে কপি করে, তারপর `d2` mutate করে।

```python
words = ["Hello"]
d = Document(words)

d2 = Document(d.get_words())
d2.add_word("world")
```

এই উদাহরণ নিয়ে দুটি মূল প্রশ্ন ভাবুন:

1. **words array কখন deallocate হয়?** 
এই প্রোগ্রাম একই array-র তিনটি pointer তৈরি করেছে। `words`, `d` আর `d2` variable-গুলোর প্রতিটিতে heap-এ বরাদ্দ করা words array-র একটি pointer আছে। তাই তিনটি variable-ই scope-এর বাইরে গেলে তবেই Python words array deallocate করবে। আরও সাধারণভাবে, শুধু সোর্স কোড পড়ে ডেটা কোথায় garbage-collect হবে তা আগে থেকে বলা প্রায়ই কঠিন।

2. **document `d`-এর বিষয়বস্তু কী?** 
`d2`-তে যেহেতু `d`-এর মতো একই words array-র pointer আছে, তাই `d2.add_word("world")` document `d`-কেও mutate করে। ফলে এই উদাহরণে `d`-এর শব্দগুলো হলো `["Hello", "world"]`। এটি ঘটে, কারণ `d.get_words()` `d`-এর words array-র একটি mutable reference return করে। data structure যখন তার ভেতরের জিনিস বাইরে ফাঁস করে দিতে পারে, তখন সর্বব্যাপী, অন্তর্নিহিত mutable reference সহজেই অপ্রত্যাশিত বাগের জন্ম দেয়[^ownership-originally]। এখানে `d2`-এর পরিবর্তন `d`-কেও বদলে দিতে পারে, এটা সম্ভবত কাঙ্ক্ষিত আচরণ নয়।

এই সমস্যা শুধু Python-এর নয় &mdash; C#, Java, Javascript ইত্যাদিতেও একই রকম আচরণের মুখোমুখি হতে পারেন। আসলে বেশিরভাগ প্রোগ্রামিং ভাষাতেই pointer-এর ধারণা আছে। প্রশ্ন শুধু ভাষাটি প্রোগ্রামারের কাছে pointer কীভাবে প্রকাশ করে। garbage collection কোন variable কোন ডেটাকে নির্দেশ করছে, তা দেখা কঠিন করে দেয়। যেমন, `d.get_words()` যে `d`-এর ভেতরের ডেটার একটি pointer তৈরি করেছে, তা স্পষ্ট ছিল না। 

অন্যদিকে Rust-এর ownership মডেল pointer-কে একেবারে সামনে নিয়ে আসে। `Document` type-কে Rust-এর একটি data structure-এ রূপান্তর করে আমরা তা দেখতে পারি। সাধারণত আমরা একটি `struct` ব্যবহার করতাম, কিন্তু সেগুলো এখনো আলোচনা করিনি, তাই শুধু একটি type alias (কোনো type-এর বিকল্প নাম) ব্যবহার করব:

```rust
type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}
```

এই Rust API কয়েকটি মূল দিক থেকে Python API থেকে আলাদা:

* `new_document` function ইনপুট vector `words`-এর ownership নিয়ে নেয়। এর মানে `Document` word vector-টির *owner*। word vector-টি তার owner `Document` scope-এর বাইরে গেলে আগে থেকে বলা যায় এমনভাবে deallocate হবে।

* document mutate করতে `add_word` function-এর একটি mutable reference `&mut Document` দরকার। এটি ইনপুট `word`-এর ownership-ও নিয়ে নেয়, অর্থাৎ document-এর আলাদা আলাদা শব্দ আর কেউ mutate করতে পারে না।

* `get_words` function document-এর ভেতরের string-গুলোর একটি স্পষ্ট immutable reference return করে। এই word vector থেকে নতুন document তৈরির একমাত্র উপায় হলো এর বিষয়বস্তু deep-copy করা, এভাবে:

```rust,ignore
fn main() {
    let words = vec!["hello".to_string()];
    let d = new_document(words);

    // .to_vec() প্রতিটি string clone করে &[String]-কে Vec<String>-এ রূপান্তর করে
    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    // `d2`-এর পরিবর্তন `d`-কে প্রভাবিত করে না
    assert!(!get_words(&d).contains(&"world".into()));
}
```

এই উদাহরণের মূল কথা হলো: Rust যদি আপনার প্রথম ভাষা না হয়, তাহলে মেমোরি আর pointer নিয়ে কাজ করার অভিজ্ঞতা আপনার আগে থেকেই আছে! Rust শুধু ধারণাগুলোকে স্পষ্ট করে দেয়। এর দ্বিমুখী সুবিধা হলো (1) garbage collection এড়িয়ে runtime পারফরম্যান্স বাড়ানো, এবং (2) ডেটার অনিচ্ছাকৃত "ফাঁস" আটকে আচরণ আগে থেকে বোঝা সহজ করা।

### ownership-এর ধারণাগুলো {#the-concepts-of-ownership}

এরপর চলুন ownership-এর ধারণাগুলো ঝালিয়ে নিই। এই পুনরালোচনা দ্রুত হবে &mdash; উদ্দেশ্য হলো প্রাসঙ্গিক ধারণাগুলো আপনাকে মনে করিয়ে দেওয়া। কোনো ধারণা ভুলে গেছেন বা বোঝেননি বলে মনে হলে আমরা প্রাসঙ্গিক অধ্যায়গুলোর লিংক দেব, যেগুলো আবার পড়ে নিতে পারেন।

#### runtime-এ ownership {#ownership-at-runtime}

শুরু করব Rust runtime-এ মেমোরি কীভাবে ব্যবহার করে তা ঝালিয়ে নিয়ে: 
* Rust local variable allocate করে stack frame-এ, যা কোনো function call হলে allocate হয় এবং call শেষ হলে deallocate হয়। 
* local variable-এ থাকতে পারে হয় ডেটা (যেমন সংখ্যা, boolean, tuple ইত্যাদি), নয়তো pointer। 
* pointer তৈরি করা যায় হয় box (heap-এর ডেটার মালিক pointer) দিয়ে, নয়তো reference (non-owning pointer) দিয়ে।

এই ডায়াগ্রামটি দেখায় runtime-এ প্রতিটি ধারণা দেখতে কেমন:

```aquascope,interpreter,horizontal
fn main() {
  let mut a_num = 0;
  inner(&mut a_num);`[]`
}

fn inner(x: &mut i32) {
  let another_num = 1;
  let a_stack_ref = &another_num;

  let a_box = Box::new(2);  
  let a_box_stack_ref = &a_box;
  let a_box_heap_ref = &*a_box;`[]`

  *x += 5;
}
```

ডায়াগ্রামটি ভালো করে দেখুন এবং নিশ্চিত হোন যে এর প্রতিটি অংশ আপনি বুঝছেন। যেমন, আপনার এই প্রশ্নগুলোর উত্তর দিতে পারা উচিত:
* `a_box_stack_ref` কেন stack-কে নির্দেশ করে, অথচ `a_box_heap_ref` heap-কে? 
* L2-তে `2` মানটি কেন আর heap-এ নেই? 
* L2-তে `a_num`-এর মান কেন `5`?

box ঝালিয়ে নিতে চাইলে [অধ্যায় 4.1][ch04-01] আবার পড়ুন। reference ঝালিয়ে নিতে চাইলে [অধ্যায় 4.2][ch04-02] আবার পড়ুন। box আর reference নিয়ে উদাহরণ-বিশ্লেষণ দেখতে চাইলে [অধ্যায় 4.3][ch04-03] আবার পড়ুন।

slice হলো এক বিশেষ ধরনের reference, যা মেমোরিতে ডেটার একটি একটানা ক্রমকে নির্দেশ করে। এই ডায়াগ্রামটি দেখায়, একটি slice কীভাবে একটি string-এর অক্ষরগুলোর একটি উপ-ক্রমকে নির্দেশ করে:

```aquascope,interpreter
fn main() {
  let s = String::from("abcdefg");
  let s_slice = &s[2..5];`[]`
}
```

slice ঝালিয়ে নিতে চাইলে [অধ্যায় 4.4][ch04-04] আবার পড়ুন।


#### compile-time-এ ownership {#ownership-at-compile-time}

Rust প্রতিটি variable-এর @Perm{read} (read), @Perm{write} (write) আর @Perm{own} (own) permission (অনুমতি)-এর হিসাব রাখে। কোনো operation করতে Rust চায় variable-টির উপযুক্ত permission থাকুক। একটি সাধারণ উদাহরণ হিসেবে, কোনো variable `let mut` হিসেবে ঘোষণা করা না থাকলে তার @Perm{write} permission থাকে না, তাই সেটি mutate করা যায় না:

```aquascope,permissions,stepper,boundaries,shouldFail
fn main() {
  let n = 0;
  n += 1;
}
```

কোনো variable **move** বা **borrow** হলে তার permission বদলাতে পারে। non-copyable type (যেমন `Box<T>` বা `String`)-এর কোনো variable move করতে @Perm{read}@Perm{own} permission লাগে, আর move-এর ফলে variable-টির সব permission মুছে যায়। এই নিয়ম move হওয়া variable-এর ব্যবহার আটকায়:

```aquascope,permissions,stepper,boundaries,shouldFail
fn main() {
  let s = String::from("Hello world");
  consume_a_string(s);
  println!("{s}"); // can't read `s` after moving it
}

fn consume_a_string(_s: String) {
  // om nom nom
}
```

move কীভাবে কাজ করে তা ঝালিয়ে নিতে চাইলে [অধ্যায় 4.1][ch04-01] আবার পড়ুন।

কোনো variable borrow করলে (অর্থাৎ এর একটি reference তৈরি করলে) সাময়িকভাবে variable-টির কিছু permission সরে যায়। একটি immutable borrow একটি immutable reference তৈরি করে, এবং borrow হওয়া ডেটা mutate বা move করা বন্ধ করে দেয়। যেমন, একটি immutable reference প্রিন্ট করা চলে:

```aquascope,permissions,stepper,boundaries
#fn main() {
let mut s = String::from("Hello");
let s_ref = &s;
println!("{s_ref}");
println!("{s}");
#}
```

কিন্তু একটি immutable reference mutate করা চলে না:

```aquascope,permissions,stepper,boundaries,shouldFail
#fn main() {
let mut s = String::from("Hello");
let s_ref = &s;`(focus,paths:*s_ref)`
s_ref.push_str(" world");
println!("{s}");
#}
```

আর immutable-ভাবে borrow হওয়া ডেটা mutate করাও চলে না:

```aquascope,permissions,stepper,boundaries,shouldFail
#fn main() {
let mut s = String::from("Hello");`(focus)`
let s_ref = &s;`(focus,rxpaths:s$)`
s.push_str(" world");
println!("{s_ref}");
#}
```

আর reference থেকে ডেটা move করে বের করাও চলে না:

```aquascope,permissions,stepper,boundaries,shouldFail
#fn main() {
let mut s = String::from("Hello");
let s_ref = &s;`(focus,paths:*s_ref)`
let s2 = *s_ref;
println!("{s}");
#}
```

একটি mutable borrow একটি mutable reference তৈরি করে, যা borrow হওয়া ডেটা পড়া, লেখা বা move করা বন্ধ করে দেয়। যেমন, একটি mutable reference mutate করা চলে:

```aquascope,permissions,stepper,boundaries
#fn main() {
let mut s = String::from("Hello");
let s_ref = &mut s;
s_ref.push_str(" world");
println!("{s}");
#}
```

কিন্তু mutable-ভাবে borrow হওয়া ডেটায় পৌঁছানো চলে না:

```aquascope,permissions,stepper,boundaries,shouldFail
#fn main() {
let mut s = String::from("Hello");
let s_ref = &mut s;`(focus,rxpaths:s$)`
println!("{s}");
s_ref.push_str(" world");
#}
```

permission আর reference ঝালিয়ে নিতে চাইলে [অধ্যায় 4.2][ch04-02] আবার পড়ুন।

#### compile-time আর runtime-এর ownership-কে যুক্ত করা {#connecting-ownership-between-compile-time-and-runtime}

Rust-এর permission-গুলোর নকশা করা হয়েছে undefined behavior আটকানোর জন্য। যেমন, এক ধরনের undefined behavior হলো **use-after-free** (free করা মেমোরি আবার ব্যবহার করা), যেখানে free করা মেমোরি পড়া বা লেখা হয়। use-after-free এড়াতে immutable borrow @Perm{write} permission সরিয়ে নেয়, যেমন এই ক্ষেত্রে:

```aquascope,interpreter,shouldFail,horizontal
#fn main() {
let mut v = vec![1, 2, 3];
let n = &v[0];`[]`
v.push(4);`[]`
println!("{n}");`[]`
#}
```

আরেক ধরনের undefined behavior হলো **double-free** (একই মেমোরি দুবার free করা), যেখানে মেমোরি দুবার free হয়। double-free এড়াতে non-copyable ডেটার reference-এর dereference-এর @Perm{own} permission থাকে না, যেমন এই ক্ষেত্রে:

```aquascope,interpreter,shouldFail,horizontal
#fn main() {
let v = vec![1, 2, 3];
let v_ref: &Vec<i32> = &v;
let v2 = *v_ref;`[]`
drop(v2);`[]`
drop(v);`[]`
#}
```

undefined behavior ঝালিয়ে নিতে চাইলে [অধ্যায় 4.1][ch04-01] আর [অধ্যায় 4.3][ch04-03] আবার পড়ুন।


### ownership-এর বাকি কথা {#the-rest-of-ownership}

struct, enum আর trait-এর মতো আরও বৈশিষ্ট্যের সঙ্গে পরিচয় করানোর সময় দেখবেন, ownership-এর সঙ্গে সেই বৈশিষ্ট্যগুলোর নির্দিষ্ট কিছু মিথস্ক্রিয়া আছে। এই অধ্যায় সেই মিথস্ক্রিয়াগুলো বোঝার অপরিহার্য ভিত্তি দেয় &mdash; মেমোরি, pointer, undefined behavior আর permission-এর ধারণাগুলো ভবিষ্যতের অধ্যায়গুলোতে Rust-এর আরও উন্নত অংশ নিয়ে কথা বলতে আমাদের সাহায্য করবে।

আর নিজের বোঝাপড়া যাচাই করতে চাইলে কুইজগুলো দিতে ভুলবেন না!

{{#quiz ../quizzes/ch04-05-ownership-recap.toml}}



[^ownership-originally]: আসলে ownership type-এর মূল উদ্ভাবন memory safety নিয়ে ছিলই না। এর উদ্দেশ্য ছিল Java-র মতো ভাষায় data structure-এর ভেতরের জিনিসের mutable reference ফাঁস হওয়া আটকানো। ownership type-এর ইতিহাস সম্পর্কে আরও জানতে আগ্রহী হলে ["Ownership Types for Flexible Alias Protection"](https://dl.acm.org/doi/abs/10.1145/286936.286947) (Clarke et al. 1998) গবেষণাপত্রটি দেখুন।

[ch04-01]: ch04-01-what-is-ownership.html
[ch04-02]: ch04-02-references-and-borrowing.html
[ch04-03]: ch04-03-fixing-ownership-errors.html
[ch04-04]: ch04-04-slices.html