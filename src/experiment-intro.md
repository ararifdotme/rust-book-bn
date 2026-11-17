# এই বইটি কোথায় আলাদা? {#whats-different-about-this-book}

<div style="display: flex; gap: 2em"> 

এই বইটি [*The Rust Programming Language*](http://doc.rust-lang.org/book/)-এর একটি পরীক্ষামূলক সংস্করণ, যা তৈরি করেছেন Brown University-র <a href="https://cel.cs.brown.edu/">Cognitive Engineering Lab</a>-এর গবেষকেরা। মূল TRPL বই থেকে এটি কোথায় আলাদা, তা জানতে চাইলে এই পাতাটি পড়ুন। আর যদি সরাসরি Rust শেখা শুরু করতে চান, তাহলে নিশ্চিন্তে এই পাতা এখন বাদ দিয়ে পরে ফিরে আসতে পারেন।

<div style="display: flex; flex-direction: column; justify-content: center">
  <img src="img/experiment/brown-logo.png" style="min-width: 150px" />
</div>

</div>


## ইন্টারঅ্যাকটিভ সুবিধা {#interactive-mechanics}

শেখার সময় আপনি যাতে Rust-এর সঙ্গে সক্রিয়ভাবে যুক্ত থাকতে পারেন, সেজন্য এই বইয়ে কিছু বাড়তি ব্যবস্থা রাখা হয়েছে। প্রথমত, নিচের মতো কুইজ দেখতে পাবেন। "Start" বোতামে ক্লিক করে একবার চেষ্টা করে দেখুন।

{{#quiz ../quizzes/example-quiz.toml}}

কোনো প্রশ্নের উত্তর ভুল হলে আপনি চাইলে কুইজটি আবার দিতে পারেন, অথবা সঠিক উত্তরগুলো দেখে নিতে পারেন। আমাদের পরামর্শ: 100% না পাওয়া পর্যন্ত কুইজটি বারবার দিন &mdash; আবার চেষ্টা করার আগে চাইলে অংশটুকু আরেকবার পড়ে নিন। তবে মনে রাখবেন, একবার সঠিক উত্তর দেখে ফেললে সেই কুইজ আর আবার দেওয়া যাবে না।

দ্বিতীয়ত, লেখার যেকোনো অংশে নিজের ভাবনা টুকে রাখতে পারেন। কিছু লেখা সিলেক্ট করে ✏️ বোতামে ক্লিক করুন, তারপর চাইলে একটি মন্তব্য লিখে দিন।

👉 এই লেখাটুকু হাইলাইট করে দেখুন! 👈

> **দ্রষ্টব্য:** আপনি যে অংশ হাইলাইট করেছেন, আমরা সেই লেখা বদলালে আপনার হাইলাইট মুছে যাবে। এছাড়া আপনার হাইলাইটগুলো কুকি হিসেবে সংরক্ষিত থাকে। কুকি ব্লক করলে বা অন্য ব্রাউজার ব্যবহার করলে আগের হাইলাইটগুলো আর দেখতে পাবেন না।

## বিষয়বস্তুতে পরিবর্তন {#content-changes}

এই বইয়ের বিষয়বস্তু মোটামুটি TRPL-এর মতোই, আর কয়েক মাস পরপর আমরা দুটি বই মিলিয়ে নিই। সবচেয়ে বড় পার্থক্য [ownership (মালিকানা) বোঝা][understanding-ownership] অধ্যায়ে। আমাদের গবেষণায় দেখা গেছে, কিছু নির্দিষ্ট ধারণা আর চিত্রের সাহায্যে ব্যাখ্যা করলে মূল বইয়ের চেয়ে Rust আরও ভালোভাবে বোঝা যায়; এই বইয়ে ownership সেভাবেই ব্যাখ্যা করা হয়েছে। নিচের মতো অনেক ডায়াগ্রাম দেখতে পাবেন। এগুলো [Aquascope][aquascope] দিয়ে তৈরি, এবং Rust কোড compile time (সংকলনের সময়) ও runtime (প্রোগ্রাম চলার সময়)-এ কী করে, তা চোখের সামনে তুলে ধরে:

```aquascope,interpreter,horizontal
#fn main() {
let mut s = String::from("Hello world");`[]`
let hello = &s[0..5];`[]`
s.push_str("!");`[]`
drop(s);`[]`
#}
```

ownership ছাড়াও, কুইজের উত্তরে পাঠকদের যেসব ভুল ধারণা চোখে পড়েছে, সেগুলো দূর করতে আমরা বইয়ে ছোটখাটো অনেক সম্পাদনা করেছি। কুইজে বা বইয়ের অন্য কোথাও কোনো সমস্যা চোখে পড়লে আমাদের Github রিপোজিটরিতে একটি issue খুলতে পারেন: <https://github.com/cognitive-engineering-lab/rust-book>

_Rust শেখা ও ব্যবহার সহজ করা নিয়ে আমাদের অন্যান্য পরীক্ষায় অংশ নিতে চান? এখানে নাম লেখান:_ <https://forms.gle/U3jEUkb2fGXykp1DA>


## প্রকাশনা {#publications}

এখন পর্যন্ত এই পরীক্ষা থেকে দুটি গবেষণাপত্র প্রকাশিত হয়েছে, যেগুলো সবাই বিনামূল্যে পড়তে পারেন। এই বইয়ের পেছনের একাডেমিক গবেষণা সম্পর্কে জানতে চাইলে পড়ে দেখুন:

* ["Profiling Programming Language Learning"](https://dl.acm.org/doi/10.1145/3649812) <br />
  [Will Crichton][will] ও [Shriram Krishnamurthi][shriram]। OOPSLA 2024। (Distinguished Paper সম্মাননাপ্রাপ্ত।)

* ["A Grounded Conceptual Model for Ownership Types in Rust"](https://dl.acm.org/doi/10.1145/3622841) <br />
  [Will Crichton][will], [Gavin Gray][gavin] ও [Shriram Krishnamurthi][shriram]। OOPSLA 2023। (SIGPLAN Research Highlight এবং Communications of the ACM Research Highlight হিসেবে নির্বাচিত।)

## কৃতজ্ঞতা {#acknowledgments}

এই কাজে আংশিক সহায়তা দিয়েছে DARPA (চুক্তি নং HR00112420354), NSF (অনুদান নং CCF-2227863) এবং Amazon Web Services। এই লেখায় প্রকাশিত সব মতামত, ফলাফল, সিদ্ধান্ত ও সুপারিশ লেখকদের নিজস্ব; এগুলো আমাদের অর্থদাতাদের মতামত নয়। পরীক্ষাটির প্রচারে সাহায্য করার জন্য Carol Nichols এবং Rust Foundation-এর কাছে আমরা কৃতজ্ঞ। আর আমরা এই পরীক্ষা শুরু করার অনেক আগেই বহু মানুষের কঠোর পরিশ্রমে TRPL গড়ে উঠেছিল।

[understanding-ownership]: ch04-00-understanding-ownership.html
[aquascope]: https://cognitive-engineering-lab.github.io/aquascope/
[will]: https://willcrichton.net/
[gavin]: https://gavinleroy.com/
[shriram]: https://cs.brown.edu/people/sk/