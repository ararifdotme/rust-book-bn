## নিজস্ব command দিয়ে Cargo-র ক্ষমতা বাড়ানো {#extending-cargo-with-custom-commands}

Cargo এমনভাবে নকশা করা যে একে পরিবর্তন না করেই নতুন subcommand দিয়ে এর ক্ষমতা বাড়ানো যায়। আপনার `$PATH`-এ থাকা কোনো binary-র নাম যদি
`cargo-something` হয়, তাহলে `cargo something` চালিয়ে একে Cargo-র একটি subcommand-এর মতো চালাতে পারেন। `cargo --list` চালালে এমন নিজস্ব command-গুলোও
তালিকায় দেখায়। `cargo install` দিয়ে extension install করে তারপর Cargo-র built-in tool-এর মতোই সেগুলো চালাতে পারা Cargo-র নকশার একটি দারুণ সুবিধা!

## সারসংক্ষেপ {#summary}

Cargo আর [crates.io](https://crates.io/)<!-- ignore --> দিয়ে কোড ভাগ করে নেওয়া Rust ecosystem-কে অনেক রকম কাজের জন্য উপযোগী করে তোলার একটি অংশ।
Rust-এর standard library ছোট ও স্থিতিশীল, কিন্তু crate ভাগ করা, ব্যবহার করা এবং ভাষার থেকে আলাদা সময়সূচিতে উন্নত করা সহজ। আপনার কাজে লাগে এমন কোড
[crates.io](https://crates.io/)<!-- ignore
-->-তে ভাগ করে নিতে দ্বিধা করবেন না; সম্ভবত সেটি অন্য কারও কাজেও লাগবে!
