# Rust প্রোগ্রামিং ভাষা {#the-rust-programming-language}

*লেখক: Steve Klabnik, Carol Nichols ও Chris Krycho; সঙ্গে Rust কমিউনিটির অবদান*

*(আর সঙ্গে কিছু পরীক্ষামূলক পরিবর্তনও!)*

এই বই ধরে নেয় যে আপনি Rust 1.90.0 (প্রকাশ: 2025-09-18) বা তার
পরের কোনো সংস্করণ ব্যবহার করছেন, এবং প্রতিটি প্রজেক্টের *Cargo.toml* ফাইলে
`edition = "2024"` লিখে প্রজেক্টগুলোকে Rust 2024 edition (Rust-এর
সংস্করণ-ধাপ)-এর idiom (Rust-এর প্রচলিত লেখার ধরন) মেনে চলার জন্য সেট করেছেন।
Rust ইনস্টল বা আপডেট করার নির্দেশনা পাবেন [অধ্যায় 1-এর “ইনস্টলেশন”
অংশে][install]<!-- ignore -->, আর edition সম্পর্কে জানতে দেখুন
[পরিশিষ্ট E][appendix-e]<!-- ignore -->।

পরীক্ষামূলক সংস্করণটি শুধু অনলাইনে এবং ইংরেজিতে পাওয়া যায়; আপনি এখন যেটি পড়ছেন, সেটি তার একটি অনানুষ্ঠানিক বাংলা অনুবাদ ([অনুবাদ সম্পর্কে][about-translation]<!-- ignore -->)। 
পরীক্ষামূলক নয় এমন মূল সংস্করণটি `rustup` দিয়ে Rust ইনস্টল করলে অফলাইনেও পাওয়া যায়; খুলতে চালান `rustup doc
--book`।

মূল সংস্করণের কমিউনিটির করা কয়েকটি [অনুবাদ][translations]ও পাওয়া যায়।
মূল সংস্করণটি No Starch Press থেকে [ছাপা বই ও ই-বুক আকারেও][nsprust]
পাওয়া যায়।

[install]: ch01-01-installation.html
[appendix-e]: appendix-05-editions.html
[nsprust]: https://nostarch.com/rust-programming-language-3rd-edition
[translations]: appendix-06-translation.html
[about-translation]: about-translation.html
