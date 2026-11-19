# অনুবাদ সম্পর্কে {#about-this-translation}

আপনি যে বইটি পড়ছেন, সেটি [*The Rust Programming Language*](https://doc.rust-lang.org/book/)-এর
Brown University-র [ইন্টারঅ্যাকটিভ সংস্করণের](https://rust-book.cs.brown.edu/) একটি অনানুষ্ঠানিক বাংলা অনুবাদ।
মূল বইটি Rust 1.90 আর Rust 2024 edition অনুযায়ী লেখা, আর এই অনুবাদও সেই সংস্করণ অনুসরণ করে।

এই অনুবাদের সঙ্গে Rust project বা Brown University-র কোনো আনুষ্ঠানিক সম্পর্ক নেই। বইয়ের সব কৃতিত্ব মূল লেখক
Steve Klabnik, Carol Nichols ও Chris Krycho, Rust কমিউনিটি এবং Brown University-র
[Cognitive Engineering Lab](https://cel.cs.brown.edu/)-এর। অনুবাদে কোনো ভুল থাকলে তার দায় অনুবাদের, মূল লেখকদের নয়।

## কীভাবে অনুবাদ করা হয়েছে {#how-it-was-translated}

অনুবাদের বড় অংশ AI-এর সাহায্যে করা হয়েছে, আর মানুষের হাতে পর্যালোচনা এখনো চলছে। তাই কোথাও কোনো বাক্য
অস্বাভাবিক, অস্পষ্ট বা ভুল মনে হলে অনুগ্রহ করে জানাবেন (কীভাবে জানাবেন, তা নিচে দেওয়া আছে)।

## অনুবাদের নীতি {#translation-approach}

পড়তে গিয়ে দেখবেন, অনেক শব্দ ইংরেজিতেই রাখা হয়েছে। এটা ইচ্ছাকৃত:

- **টেকনিক্যাল শব্দ ইংরেজিতে:** _type_, _trait_, _borrow_, _closure_, _lifetime_-এর মতো শব্দ বাংলায় বদলানো
  হয়নি। compiler-এর error message, Rust-এর documentation আর ডেভেলপারদের নিজেদের কথাবার্তায় এই শব্দগুলোই
  ব্যবহার হয়। বাংলা প্রতিশব্দ শিখলে পরে সেগুলো আবার ইংরেজির সঙ্গে মেলাতে হতো। কোনো শব্দ প্রথমবার এলে
  সাধারণত বন্ধনীতে তার বাংলা ব্যাখ্যা দেওয়া আছে।
- **কোড ও output মূলের মতোই:** কোড, terminal-এর output আর compiler-এর error message ইংরেজিতেই আছে, কারণ
  নিজের কম্পিউটারে আপনি হুবহু এগুলোই দেখবেন। কিছু উদাহরণের কোডের ভেতরের comment বাংলায় অনুবাদ করা হয়েছে।
- **নম্বর ও লেবেল:** “Listing 5-1”, “Figure 4-1”, “Table B-1” আর পরিশিষ্টের অক্ষরগুলো (A–G) ইংরেজিতে রাখা
  হয়েছে, যাতে মূল ইংরেজি বইয়ের সঙ্গে মিলিয়ে পড়া সহজ হয়।
- **কুইজ:** কুইজের প্রশ্ন, উত্তর আর ব্যাখ্যা বাংলায় অনুবাদ করা হয়েছে, তবে প্রশ্নের ভেতরের কোড ইংরেজিতেই আছে।

কোন ইংরেজি শব্দের জন্য কোন বাংলা শব্দ বা ব্যাখ্যা ব্যবহার করা হয়েছে, তার পূর্ণ তালিকা দেখতে পাবেন
[শব্দকোষে](https://github.com/ararifdotme/rust-book-bn/blob/main/TRANSLATION.md)।

## ভুল পেলে বা পরামর্শ থাকলে {#reporting-mistakes}

অনুবাদে ভুল, অস্পষ্ট বাক্য বা আরও ভালো শব্দের পরামর্শ থাকলে GitHub-এ একটি
[issue খুলুন](https://github.com/ararifdotme/rust-book-bn/issues)। পারলে অধ্যায়ের নাম আর বাক্যটি উল্লেখ করবেন।
সরাসরি সংশোধন করে pull request পাঠালেও স্বাগত জানাই। অনুবাদের source code পাবেন
[GitHub repository-তে](https://github.com/ararifdotme/rust-book-bn)।

## লাইসেন্স {#license}

মূল বইয়ের মতো এই অনুবাদও MIT ও Apache 2.0 লাইসেন্সের অধীনে প্রকাশিত। বিস্তারিত জানতে repository-র
[`LICENSE-MIT`](https://github.com/ararifdotme/rust-book-bn/blob/main/LICENSE-MIT) আর
[`LICENSE-APACHE`](https://github.com/ararifdotme/rust-book-bn/blob/main/LICENSE-APACHE) ফাইল দেখুন।
