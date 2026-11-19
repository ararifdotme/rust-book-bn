<!-- Old headings. Do not remove or links may break. -->

<a id="writing-error-messages-to-standard-error-instead-of-standard-output"></a>

## error-কে standard error-এ পাঠানো {#redirecting-errors-to-standard-error}

এই মুহূর্তে আমরা `println!` macro দিয়ে আমাদের সব output terminal-এ লিখছি। বেশিরভাগ terminal-এ দুই ধরনের output থাকে: সাধারণ তথ্যের
জন্য _standard
output_ (`stdout`) আর error message-এর জন্য _standard error_ (`stderr`)। এই পার্থক্যের কারণে ব্যবহারকারী একটি প্রোগ্রামের সফল output
একটি ফাইলে পাঠিয়ে দিয়েও error message-গুলো স্ক্রিনে print করাতে পারে।

`println!` macro শুধু standard output-এ print করতে পারে, তাই standard error-এ print করতে আমাদের অন্য কিছু ব্যবহার করতে হবে।

### error কোথায় লেখা হচ্ছে তা যাচাই করা {#checking-where-errors-are-written}

প্রথমে দেখা যাক `minigrep`-এর print করা বিষয়বস্তু এখন কীভাবে standard output-এ লেখা হচ্ছে, এমনকি সেই error message-গুলোও, যা আমরা
বরং standard error-এ লিখতে চাই। এটা দেখতে আমরা ইচ্ছে করে একটি error ঘটিয়ে standard output stream-কে একটি ফাইলে পাঠাব। standard
error stream-কে আমরা অন্য কোথাও পাঠাব না, তাই standard error-এ পাঠানো যেকোনো বিষয়বস্তু স্ক্রিনেই দেখা যেতে থাকবে।

command line প্রোগ্রামের কাছে প্রত্যাশা হলো এরা error message standard error stream-এ পাঠাবে, যাতে standard output stream-কে একটি
ফাইলে পাঠালেও আমরা স্ক্রিনে error message দেখতে পাই। আমাদের প্রোগ্রাম এখন ঠিকমতো আচরণ করছে না: আমরা এখনই দেখব যে এটি বরং error
message-এর output একটি ফাইলে সংরক্ষণ করে!

এই আচরণ দেখাতে আমরা `>` আর যে file path-এ standard output stream পাঠাতে চাই সেই _output.txt_ দিয়ে প্রোগ্রাম চালাব। কোনো argument
দেব না, যার ফলে একটি error ঘটার কথা:

```console
$ cargo run > output.txt
```

`>` syntax shell-কে বলে standard output-এর বিষয়বস্তু স্ক্রিনের বদলে _output.txt_-এ লিখতে। আমরা যে error message স্ক্রিনে print হবে
বলে আশা করছিলাম তা দেখিনি, অর্থাৎ সেটি নিশ্চয়ই ফাইলে গিয়ে পৌঁছেছে। _output.txt_-এ এটি আছে:

```text
Problem parsing arguments: not enough arguments
```

হ্যাঁ, আমাদের error message standard output-এ print হচ্ছে। এ ধরনের error message standard error-এ print হলে অনেক বেশি কাজে লাগে,
যাতে শুধু সফলভাবে চলার ডেটাই ফাইলে যায়। আমরা এটি বদলাব।

### error-কে standard error-এ print করা {#printing-errors-to-standard-error}

error message কীভাবে print হবে তা বদলাতে আমরা Listing 12-24-এর কোড ব্যবহার করব। এই অধ্যায়ের আগের দিকে করা refactoring-এর কারণে
error message print করা সব কোড একটি function-এ, `main`-এ আছে। standard library `eprintln!` macro দেয়, যা standard error stream-এ
print করে, তাই error print করতে যে দুই জায়গায় `println!` call করছিলাম, সেখানে এর বদলে `eprintln!` ব্যবহার করি।

<Listing number="12-24" file-name="src/main.rs" caption="`eprintln!` ব্যবহার করে error message standard output-এর বদলে standard error-এ লেখা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

</Listing>

এবার একইভাবে, কোনো argument ছাড়া এবং `>` দিয়ে standard output অন্যত্র পাঠিয়ে, প্রোগ্রামটি আবার চালাই:

```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

এখন আমরা স্ক্রিনে error দেখছি এবং _output.txt_-এ কিছুই নেই, যা command line প্রোগ্রামের কাছে প্রত্যাশিত আচরণ।

এবার এমন argument দিয়ে প্রোগ্রামটি আবার চালাই যাতে error হয় না, কিন্তু standard output তখনো একটি ফাইলে পাঠাই, এভাবে:

```console
$ cargo run -- to poem.txt > output.txt
```

terminal-এ কোনো output দেখব না, আর _output.txt_-এ আমাদের ফল থাকবে:

<span class="filename">Filename: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

এটি দেখায় যে আমরা এখন যথাযথভাবে সফল output-এর জন্য standard output আর error output-এর জন্য standard error ব্যবহার করছি।

## সারসংক্ষেপ {#summary}

এই অধ্যায়ে এ পর্যন্ত শেখা কিছু প্রধান ধারণার পুনরালোচনা হয়েছে, আর Rust-এ সাধারণ I/O operation কীভাবে করতে হয় তা দেখানো হয়েছে।
command line argument, ফাইল, environment variable আর error print করার জন্য `eprintln!` macro ব্যবহার করে আপনি এখন command line
application লিখতে প্রস্তুত। আগের অধ্যায়গুলোর ধারণার সঙ্গে মিলিয়ে আপনার কোড ভালোভাবে গোছানো থাকবে, উপযুক্ত data structure-এ
কার্যকরভাবে ডেটা রাখবে, সুন্দরভাবে error সামলাবে, এবং ভালোভাবে test করা থাকবে।

এরপর আমরা Rust-এর এমন কিছু বৈশিষ্ট্য দেখব যা functional ভাষা দ্বারা প্রভাবিত: closure আর iterator।

