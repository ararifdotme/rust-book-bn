# functional ভাষার বৈশিষ্ট্য: iterator ও closure {#functional-language-features-iterators-and-closures}

Rust-এর নকশা বিদ্যমান অনেক ভাষা ও কৌশল থেকে অনুপ্রেরণা নিয়েছে, আর এর একটি উল্লেখযোগ্য প্রভাব হলো _functional programming_
(ফাংশনভিত্তিক প্রোগ্রামিং)। functional ধাঁচে প্রোগ্রামিংয়ে প্রায়ই function-কে মান হিসেবে ব্যবহার করা হয়: argument হিসেবে পাঠানো,
অন্য function থেকে return করা, পরে চালানোর জন্য variable-এ রাখা, ইত্যাদি।

এই অধ্যায়ে functional programming কী আর কী নয়, তা নিয়ে বিতর্কে যাব না, বরং Rust-এর এমন কিছু বৈশিষ্ট্য নিয়ে আলোচনা করব যা প্রায়ই
functional বলে পরিচিত অনেক ভাষার বৈশিষ্ট্যের মতো।

আরও নির্দিষ্ট করে বললে, আমরা দেখব:

- _Closures_ (closure), function-এর মতো একটি গঠন যা variable-এ রাখা যায়
- _Iterators_ (iterator), একের পর এক উপাদান প্রক্রিয়া করার একটি উপায়
- অধ্যায় 12-এর I/O project উন্নত করতে closure ও iterator কীভাবে ব্যবহার করা যায়
- closure ও iterator-এর performance (আগেভাগে বলে দিই: এরা আপনার ধারণার চেয়েও দ্রুত!)

pattern matching আর enum-এর মতো Rust-এর আরও কিছু বৈশিষ্ট্য আমরা আগেই দেখেছি, যেগুলোও functional ধাঁচ দ্বারা প্রভাবিত। দ্রুত ও
idiomatic Rust কোড লেখার একটি গুরুত্বপূর্ণ অংশ হলো closure ও iterator আয়ত্ত করা, তাই এই পুরো অধ্যায়টি আমরা এদের জন্যই রাখছি।
