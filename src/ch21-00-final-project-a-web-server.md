# শেষ project: একটি multithreaded web server তৈরি {#final-project-building-a-multithreaded-web-server}

লম্বা যাত্রা ছিল, তবে আমরা বইয়ের শেষে পৌঁছে গেছি। এই অধ্যায়ে আমরা একসঙ্গে আরও একটি project বানাব, যাতে শেষ অধ্যায়গুলোতে আলোচিত কিছু ধারণা দেখানো যায়, আর আগের কিছু পাঠেরও পুনরালোচনা হয়।

আমাদের শেষ project-এ আমরা এমন একটি web server বানাব, যা “Hello!” বলে এবং web browser-এ দেখতে Figure 21-1-এর মতো।

web server বানানোর জন্য আমাদের পরিকল্পনা এই:

1. TCP আর HTTP সম্পর্কে একটু জানা।
2. একটি socket-এ TCP connection-এর জন্য অপেক্ষা করা (listen করা)।
3. অল্প কয়েকটি HTTP request parse করা।
4. একটি যথাযথ HTTP response তৈরি করা।
5. একটি thread pool দিয়ে আমাদের server-এর throughput (একই সময়ে কতটা কাজ সামলাতে পারে) বাড়ানো।

<img alt="একটি web browser-এর screenshot, যা 127.0.0.1:8080 ঠিকানায় গিয়ে “Hello! Hi from Rust” লেখাসহ একটি webpage দেখাচ্ছে" src="img/trpl21-01.png" class="center" style="width: 50%;" />

<span class="caption">Figure 21-1: আমাদের শেষ যৌথ project</span>

শুরু করার আগে দুটি বিষয় উল্লেখ করা উচিত। প্রথমত, আমরা যে পদ্ধতি ব্যবহার করব তা Rust দিয়ে web server বানানোর সেরা উপায় হবে না। community-র সদস্যরা [crates.io](https://crates.io/)-তে production-এর
উপযোগী অনেক crate প্রকাশ করেছেন, যা আমরা যা বানাব তার চেয়ে পূর্ণাঙ্গ web server আর thread pool implementation দেয়। তবে এই অধ্যায়ে আমাদের উদ্দেশ্য আপনাকে শিখতে সাহায্য করা, সহজ পথ নেওয়া নয়। Rust
যেহেতু একটি systems programming ভাষা, তাই কোন স্তরের abstraction নিয়ে কাজ করব তা বেছে নিতে পারি, এবং অন্য ভাষায় যা সম্ভব বা বাস্তবসম্মত তার চেয়ে নিচের স্তরে যেতে পারি।

দ্বিতীয়ত, এখানে আমরা async আর await ব্যবহার করব না। একটি async runtime বানানো যোগ না করেই একটি thread pool বানানো নিজেই যথেষ্ট বড় চ্যালেঞ্জ! তবে এই অধ্যায়ে আমরা যে সমস্যাগুলো দেখব, তার কিছুতে
async আর await কীভাবে প্রযোজ্য হতে পারে তা উল্লেখ করব। শেষ পর্যন্ত, অধ্যায় 17-এ যেমন বলেছিলাম, অনেক async runtime তাদের কাজ সামলাতে thread pool ব্যবহার করে।

তাই আমরা মৌলিক HTTP server আর thread pool হাতে লিখব, যাতে ভবিষ্যতে আপনি যে crate-গুলো ব্যবহার করতে পারেন সেগুলোর পেছনের সাধারণ ধারণা ও কৌশল শিখতে পারেন।
