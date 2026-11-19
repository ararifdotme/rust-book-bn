# Cargo ও Crates.io সম্পর্কে আরও {#more-about-cargo-and-cratesio}

এ পর্যন্ত আমরা কোড build, run আর test করতে Cargo-র শুধু সবচেয়ে মৌলিক বৈশিষ্ট্যগুলো ব্যবহার করেছি, কিন্তু এটি আরও অনেক কিছু করতে পারে। এই
অধ্যায়ে আমরা এর আরও কিছু উন্নত বৈশিষ্ট্য নিয়ে আলোচনা করব, যাতে দেখাতে পারি কীভাবে এগুলো করতে হয়:

- release profile দিয়ে আপনার build নিজের মতো সাজানো।
- [crates.io](https://crates.io/)<!-- ignore -->-তে library প্রকাশ করা।
- workspace দিয়ে বড় project গুছিয়ে রাখা।
- [crates.io](https://crates.io/)<!-- ignore --> থেকে binary install করা।
- নিজস্ব command দিয়ে Cargo-র ক্ষমতা বাড়ানো।

এই অধ্যায়ে আলোচিত কার্যক্ষমতার চেয়েও বেশি কিছু Cargo করতে পারে, তাই এর সব বৈশিষ্ট্যের পূর্ণ ব্যাখ্যার জন্য [এর documentation](https://doc.rust-lang.org/cargo/) দেখুন।
