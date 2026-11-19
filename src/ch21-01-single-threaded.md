## একটি single-threaded web server তৈরি করা {#building-a-single-threaded-web-server}

আমরা শুরু করব একটি single-threaded web server চালু করে। শুরু করার আগে web server বানাতে জড়িত protocol (যোগাযোগের নিয়ম)-গুলোর একটি দ্রুত পরিচিতি দেখা যাক। এই protocol-গুলোর খুঁটিনাটি এই বইয়ের
আওতার বাইরে, তবে একটি সংক্ষিপ্ত পরিচিতি আপনাকে দরকারি তথ্য দেবে।

web server-এ জড়িত দুটি প্রধান protocol হলো _Hypertext Transfer
Protocol_ _(HTTP)_ আর _Transmission Control Protocol_ _(TCP)_। দুটি protocol-ই _request-response_ (অনুরোধ-উত্তর) protocol, অর্থাৎ একটি _client_ request শুরু করে, আর একটি _server_ request শোনে এবং
client-কে একটি response দেয়। সেই request আর response-এর বিষয়বস্তু protocol-গুলো সংজ্ঞায়িত করে।

TCP হলো নিম্ন-স্তরের protocol, যা তথ্য কীভাবে এক server থেকে আরেক server-এ যায় তার খুঁটিনাটি বর্ণনা করে, কিন্তু সেই তথ্য কী তা নির্দিষ্ট করে না। HTTP request আর response-এর বিষয়বস্তু সংজ্ঞায়িত করে TCP-এর
ওপর ভিত্তি করে তৈরি। কারিগরি দিক থেকে অন্য protocol-এর সঙ্গেও HTTP ব্যবহার করা সম্ভব, কিন্তু বেশিরভাগ ক্ষেত্রে HTTP তার ডেটা TCP দিয়েই পাঠায়। আমরা TCP আর HTTP request ও response-এর কাঁচা byte নিয়ে
কাজ করব।

### TCP connection-এর জন্য অপেক্ষা করা {#listening-to-the-tcp-connection}

আমাদের web server-কে একটি TCP connection-এর জন্য অপেক্ষা করতে (listen করতে) হবে, তাই আমরা প্রথমে এই অংশে কাজ করব। standard library একটি `std::net` module দেয়, যা দিয়ে এটা করা যায়। চলুন স্বাভাবিক উপায়ে
একটি নতুন project বানাই:

```console
$ cargo new hello
     Created binary (application) `hello` project
$ cd hello
```

এবার শুরু করতে _src/main.rs_-এ Listing 21-1-এর কোড লিখুন। এই কোড আসতে থাকা TCP stream-এর জন্য local ঠিকানা `127.0.0.1:7878`-এ অপেক্ষা করবে। একটি stream এলে এটি print করবে `Connection established!`।

<Listing number="21-1" file-name="src/main.rs" caption="আসতে থাকা stream-এর জন্য অপেক্ষা করা এবং একটি stream পেলে একটি message print করা">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-01/src/main.rs}}
```

</Listing>

`TcpListener` ব্যবহার করে আমরা `127.0.0.1:7878` ঠিকানায় TCP connection-এর জন্য অপেক্ষা করতে পারি। ঠিকানায় colon-এর আগের অংশটি আপনার কম্পিউটার বোঝানো একটি IP ঠিকানা (এটি প্রতিটি কম্পিউটারে একই, আর
বিশেষভাবে লেখকদের কম্পিউটার বোঝায় না), আর `7878` হলো port। এই port দুটি কারণে বেছে নিয়েছি: সাধারণত এই port-এ HTTP গ্রহণ করা হয় না, তাই আপনার যন্ত্রে চলা অন্য কোনো web server-এর সঙ্গে আমাদের server-এর
সংঘাতের সম্ভাবনা কম, আর টেলিফোনে _rust_ টাইপ করলে হয় 7878।

এই পরিস্থিতিতে `bind` function `new` function-এর মতো কাজ করে, কারণ এটি একটি নতুন `TcpListener` instance return করবে। function-টির নাম `bind`, কারণ networking-এ অপেক্ষা করার জন্য একটি port-এর সঙ্গে
যুক্ত হওয়াকে বলা হয় “port-এর সঙ্গে bind করা।”

`bind` function একটি `Result<T, E>` return করে, যা বোঝায় bind করা ব্যর্থ হতে পারে, যেমন যদি আমাদের প্রোগ্রামের দুটি instance চালাতাম এবং ফলে একই port-এ দুটি প্রোগ্রাম অপেক্ষা করত। আমরা যেহেতু শুধু
শেখার উদ্দেশ্যে একটি মৌলিক server লিখছি, তাই এ ধরনের error সামলানো নিয়ে চিন্তা করব না; বরং error হলে প্রোগ্রাম থামাতে `unwrap` ব্যবহার করি।

`TcpListener`-এর `incoming` method একটি iterator return করে, যা আমাদের stream-এর একটি ধারা দেয় (আরও নির্দিষ্ট করে বললে `TcpStream` type-এর stream)। একটি _stream_ client আর server-এর মধ্যে একটি খোলা
connection বোঝায়। _Connection_ হলো পুরো request আর response প্রক্রিয়ার নাম, যেখানে একটি client server-এর সঙ্গে যুক্ত হয়, server একটি response তৈরি করে, আর server connection বন্ধ করে। তাই client কী
পাঠিয়েছে তা দেখতে আমরা `TcpStream` থেকে পড়ব, তারপর client-কে ডেটা ফেরত পাঠাতে stream-এ আমাদের response লিখব। সামগ্রিকভাবে এই `for` loop প্রতিটি connection একে একে প্রক্রিয়া করবে এবং আমাদের সামলানোর
জন্য stream-এর একটি ধারা তৈরি করবে।

আপাতত stream সামলানো বলতে stream-এ কোনো error থাকলে প্রোগ্রাম শেষ করতে `unwrap` call করা; কোনো error না থাকলে প্রোগ্রাম একটি message print করে। পরের listing-এ সফলতার ক্ষেত্রের জন্য আরও কার্যক্ষমতা যোগ
করব। একটি client server-এর সঙ্গে যুক্ত হলে `incoming` method থেকে error পেতে পারার কারণ হলো আমরা আসলে connection-এর ওপর দিয়ে iterate করছি না। বরং iterate করছি _connection-এর চেষ্টার_ ওপর দিয়ে। নানা
কারণে connection সফল না-ও হতে পারে, যার অনেকগুলো operating system-নির্দিষ্ট। যেমন, অনেক operating system একসঙ্গে কতগুলো connection খোলা রাখতে পারবে তার একটি সীমা থাকে; সেই সংখ্যার বেশি নতুন
connection-এর চেষ্টা error দেবে, যতক্ষণ না কিছু খোলা connection বন্ধ হয়।

চলুন এই কোড চালিয়ে দেখি! terminal-এ `cargo run` চালিয়ে তারপর একটি web browser-এ
_127.0.0.1:7878_ load করুন। browser “Connection reset”-এর মতো একটি error message দেখানোর কথা, কারণ server এখন কোনো ডেটা ফেরত পাঠাচ্ছে না। কিন্তু terminal-এ তাকালে দেখবেন browser server-এর সঙ্গে যুক্ত
হওয়ার সময় print হওয়া কয়েকটি message!

```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

কখনো কখনো একটি browser request-এর জন্য একাধিক message print হতে দেখবেন; কারণ হতে পারে browser page-এর জন্য request করার পাশাপাশি অন্য resource-এর জন্যও request করছে, যেমন browser tab-এ দেখা
_favicon.ico_ icon।

এমনও হতে পারে যে server কোনো ডেটা দিয়ে সাড়া দিচ্ছে না বলে browser একাধিকবার server-এর সঙ্গে যুক্ত হওয়ার চেষ্টা করছে। loop-এর শেষে `stream` scope-এর বাইরে গিয়ে drop হলে `drop` implementation-এর অংশ
হিসেবে connection বন্ধ হয়ে যায়। browser কখনো কখনো বন্ধ connection আবার চেষ্টা করে সামলায়, কারণ সমস্যাটি হয়তো সাময়িক।

browser কখনো কখনো কোনো request না পাঠিয়েই server-এর সঙ্গে একাধিক connection খোলে, যাতে পরে request _পাঠালে_ সেগুলো আরও দ্রুত হতে পারে। এমন হলে আমাদের server প্রতিটি connection দেখবে, সেই
connection-এ কোনো request থাকুক বা না থাকুক। যেমন, Chrome-ভিত্তিক অনেক browser-এর সংস্করণ এটা করে; private browsing mode ব্যবহার করে বা ভিন্ন browser ব্যবহার করে সেই optimization বন্ধ করতে পারেন।

গুরুত্বপূর্ণ বিষয় হলো আমরা সফলভাবে একটি TCP connection-এর handle পেয়েছি!

কোডের একটি নির্দিষ্ট সংস্করণ চালানো শেষ হলে <kbd>ctrl</kbd>-<kbd>C</kbd> চেপে প্রোগ্রাম থামাতে ভুলবেন না। তারপর প্রতিবার কোডে পরিবর্তন আনার পর `cargo run` command চালিয়ে প্রোগ্রাম আবার শুরু করুন,
যাতে নিশ্চিত হন যে সবচেয়ে নতুন কোড চালাচ্ছেন।

### request পড়া {#reading-the-request}

চলুন browser থেকে request পড়ার কার্যক্ষমতা implement করি! প্রথমে একটি connection পাওয়া আর তারপর connection নিয়ে কিছু করা, এই দুটি দায়িত্ব আলাদা করতে আমরা connection প্রক্রিয়ার জন্য একটি নতুন
function শুরু করব। এই নতুন `handle_connection` function-এ আমরা TCP stream থেকে ডেটা পড়ে print করব, যাতে browser থেকে পাঠানো ডেটা দেখতে পারি। কোড বদলে Listing 21-2-এর মতো করুন।

<Listing number="21-2" file-name="src/main.rs" caption="`TcpStream` থেকে পড়ে ডেটা print করা">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-02/src/main.rs}}
```

</Listing>

stream থেকে পড়া ও তাতে লেখার trait আর type পেতে আমরা `std::io::BufReader` আর `std::io::prelude` scope-এ আনি। `main` function-এর `for` loop-এ connection তৈরি হয়েছে বলে message print না করে এখন
নতুন `handle_connection` function call করে তাকে `stream` পাঠাই।

`handle_connection` function-এ আমরা একটি নতুন `BufReader` instance তৈরি করি, যা `stream`-এর একটি reference মুড়ে রাখে। `BufReader` আমাদের হয়ে `std::io::Read` trait-এর method call সামলে buffering (একসঙ্গে
জমিয়ে পড়া) যোগ করে।

browser আমাদের server-এ request-এর যে লাইনগুলো পাঠায় সেগুলো জড়ো করতে আমরা `http_request` নামের একটি variable তৈরি করি। `Vec<_>` type annotation যোগ করে বোঝাই যে এই লাইনগুলো একটি vector-এ জড়ো করতে চাই।

`BufReader` `std::io::BufRead` trait implement করে, যা `lines` method দেয়। `lines` method ডেটার stream-এ newline byte দেখলেই ভাগ করে `Result<String,
std::io::Error>`-এর একটি iterator return করে। প্রতিটি `String` পেতে আমরা প্রতিটি `Result`-এর ওপর `map` আর `unwrap` করি। ডেটা বৈধ UTF-8 না হলে বা stream থেকে পড়তে সমস্যা হলে `Result` একটি error হতে
পারে। আবারও, production প্রোগ্রামের এই error-গুলো আরও সুন্দরভাবে সামলানো উচিত, তবে সরলতার জন্য আমরা error-এর ক্ষেত্রে প্রোগ্রাম থামানোর পথ বেছে নিচ্ছি।

browser পরপর দুটি newline অক্ষর পাঠিয়ে একটি HTTP request-এর শেষ জানায়, তাই stream থেকে একটি request পেতে আমরা লাইন নিতে থাকি, যতক্ষণ না খালি string-এর একটি লাইন পাই। লাইনগুলো vector-এ জড়ো করার পর
সেগুলো সুন্দর debug formatting দিয়ে print করি, যাতে web browser আমাদের server-এ কী নির্দেশ পাঠাচ্ছে তা দেখতে পারি।

চলুন এই কোড চালিয়ে দেখি! প্রোগ্রাম চালু করে আবার একটি web browser-এ request করুন। খেয়াল করুন, browser-এ তবুও একটি error page পাব, তবে terminal-এ আমাদের প্রোগ্রামের output এখন মোটামুটি এমন দেখাবে:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-02
cargo run
make a request to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

আপনার browser অনুযায়ী output একটু আলাদা হতে পারে। এখন request-এর ডেটা print করছি বলে request-এর প্রথম লাইনে `GET`-এর পরের path দেখে বুঝতে পারি কেন একটি browser request থেকে একাধিক connection পাই।
বারবার আসা connection-গুলো যদি সবই _/_ চায়, তাহলে বুঝব browser বারবার _/_ আনার চেষ্টা করছে, কারণ আমাদের প্রোগ্রাম থেকে কোনো response পাচ্ছে না।

browser আমাদের প্রোগ্রামের কাছে কী চাইছে তা বুঝতে চলুন এই request-এর ডেটা বিশ্লেষণ করি।

<!-- Old headings. Do not remove or links may break. -->

<a id="a-closer-look-at-an-http-request"></a>
<a id="looking-closer-at-an-http-request"></a>

### একটি HTTP request আরও কাছ থেকে দেখা {#looking-more-closely-at-an-http-request}

HTTP একটি লেখা-ভিত্তিক protocol, আর একটি request-এর ধরন এমন:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

প্রথম লাইনটি হলো _request line_, যাতে client কী চাইছে সেই তথ্য থাকে। request line-এর প্রথম অংশ ব্যবহৃত method বোঝায়, যেমন `GET` বা `POST`, যা বর্ণনা করে client কীভাবে এই request করছে। আমাদের client
একটি `GET` request ব্যবহার করেছে, অর্থাৎ এটি তথ্য চাইছে।

request line-এর পরের অংশ হলো _/_, যা বোঝায় client কোন _uniform resource
identifier_ _(URI)_ চাইছে: একটি URI প্রায়, তবে পুরোপুরি নয়, একটি _uniform resource locator_ _(URL)_-এর মতো। এই অধ্যায়ে আমাদের উদ্দেশ্যে URI আর URL-এর পার্থক্য গুরুত্বপূর্ণ নয়, তবে HTTP-র বিবরণে
_URI_ পরিভাষা ব্যবহৃত হয়, তাই এখানে মনে মনে _URI_-এর জায়গায় _URL_ বসিয়ে নিতে পারেন।

শেষ অংশটি হলো client-এর ব্যবহৃত HTTP version, তারপর request line শেষ হয় একটি CRLF ক্রম দিয়ে। (_CRLF_-এর পূর্ণরূপ _carriage return_ আর _line feed_, যা টাইপরাইটারের যুগের পরিভাষা!) CRLF ক্রমকে
`\r\n` হিসেবেও লেখা যায়, যেখানে `\r` হলো carriage return আর `\n` হলো line feed। _CRLF sequence_ request line-কে request-এর বাকি ডেটা থেকে আলাদা করে। খেয়াল করুন, CRLF print হলে আমরা `\r\n`
দেখি না, বরং একটি নতুন লাইন শুরু হতে দেখি।

এ পর্যন্ত আমাদের প্রোগ্রাম চালিয়ে পাওয়া request line-এর ডেটা দেখলে দেখি `GET` হলো method, _/_ হলো request URI, আর `HTTP/1.1` হলো version।

request line-এর পরে `Host:` থেকে শুরু হওয়া বাকি লাইনগুলো হলো header। `GET` request-এর কোনো body থাকে না।

ভিন্ন একটি browser থেকে request করে, বা ভিন্ন একটি ঠিকানা, যেমন _127.0.0.1:7878/test_, চেয়ে দেখুন request-এর ডেটা কীভাবে বদলায়।

browser কী চাইছে তা জানলাম, এবার কিছু ডেটা ফেরত পাঠাই!

### একটি response লেখা {#writing-a-response}

আমরা একটি client request-এর উত্তরে ডেটা পাঠানো implement করতে যাচ্ছি। response-এর ধরন এমন:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

প্রথম লাইনটি একটি _status line_, যাতে response-এ ব্যবহৃত HTTP version, request-এর ফলাফলের সারসংক্ষেপ দেওয়া একটি সংখ্যার status code, আর status code-এর লেখায় বর্ণনা দেওয়া একটি reason phrase থাকে।
CRLF ক্রমের পরে থাকে যেকোনো header, আরেকটি CRLF ক্রম, আর response-এর body।

এখানে একটি উদাহরণ response দেওয়া হলো, যা HTTP version 1.1 ব্যবহার করে, যার status code 200, reason phrase OK, কোনো header নেই, আর কোনো body নেই:

```text
HTTP/1.1 200 OK\r\n\r\n
```

status code 200 হলো সফলতার আদর্শ response। লেখাটি একটি ক্ষুদ্র সফল HTTP response। চলুন একটি সফল request-এর উত্তর হিসেবে এটি stream-এ লিখি! `handle_connection` function থেকে request-এর ডেটা
print করা `println!` সরিয়ে তার জায়গায় Listing 21-3-এর কোড বসান।

<Listing number="21-3" file-name="src/main.rs" caption="stream-এ একটি ক্ষুদ্র সফল HTTP response লেখা">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-03/src/main.rs:here}}
```

</Listing>

প্রথম নতুন লাইনটি `response` variable সংজ্ঞায়িত করে, যাতে সফলতার message-এর ডেটা থাকে। তারপর string ডেটাকে byte-এ রূপান্তর করতে আমাদের `response`-এর ওপর `as_bytes` call করি। `stream`-এর ওপর
`write_all` method একটি `&[u8]` নিয়ে সেই byte-গুলো সরাসরি connection দিয়ে পাঠায়। `write_all` operation ব্যর্থ হতে পারে বলে আগের মতোই যেকোনো error ফলের ওপর `unwrap` ব্যবহার করি। আবারও, একটি
বাস্তব application-এ এখানে error handling যোগ করতেন।

এই পরিবর্তনগুলোসহ চলুন কোড চালিয়ে একটি request করি। আমরা আর terminal-এ কোনো ডেটা print করছি না, তাই Cargo-র output ছাড়া আর কোনো output দেখব না। একটি web browser-এ _127.0.0.1:7878_ load
করলে error-এর বদলে একটি ফাঁকা page পাওয়ার কথা। আপনি এইমাত্র হাতে কোড লিখে একটি HTTP request গ্রহণ করে একটি response পাঠালেন!

### আসল HTML return করা {#returning-real-html}

চলুন একটি ফাঁকা page-এর চেয়ে বেশি কিছু return করার কার্যক্ষমতা implement করি। _src_ directory-তে নয়, আপনার project directory-র মূলে _hello.html_ নামে নতুন ফাইল তৈরি করুন। যেকোনো HTML লিখতে পারেন;
Listing 21-4-এ একটি সম্ভাবনা দেখানো হয়েছে।

<Listing number="21-4" file-name="hello.html" caption="একটি response-এ return করার জন্য একটি নমুনা HTML ফাইল">

```html
{{#include ../listings/ch21-web-server/listing-21-05/hello.html}}
```

</Listing>

এটি একটি শিরোনাম আর কিছু লেখাসহ একটি ন্যূনতম HTML5 document। একটি request পেলে server থেকে এটি return করতে আমরা `handle_connection` বদলাব, যেমনটি Listing 21-5-এ দেখানো হয়েছে, যাতে এটি HTML ফাইলটি পড়ে,
response-এ body হিসেবে যোগ করে, এবং পাঠায়।

<Listing number="21-5" file-name="src/main.rs" caption="response-এর body হিসেবে *hello.html*-এর বিষয়বস্তু পাঠানো">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-05/src/main.rs:here}}
```

</Listing>

standard library-র filesystem module scope-এ আনতে আমরা `use` statement-এ `fs` যোগ করেছি। একটি ফাইলের বিষয়বস্তু একটি string-এ পড়ার কোড পরিচিত লাগার কথা; Listing 12-4-এ আমাদের I/O project-এর
জন্য একটি ফাইলের বিষয়বস্তু পড়ার সময় এটি ব্যবহার করেছিলাম।

এরপর সফলতার response-এর body হিসেবে ফাইলের বিষয়বস্তু যোগ করতে `format!` ব্যবহার করি। একটি বৈধ HTTP response নিশ্চিত করতে আমরা `Content-Length` header যোগ করি, যার মান আমাদের response body-র আকার—এই
ক্ষেত্রে `hello.html`-এর আকার।

`cargo run` দিয়ে এই কোড চালিয়ে browser-এ _127.0.0.1:7878_ load করুন; আপনার HTML দেখানো হতে দেখবেন!

এই মুহূর্তে আমরা `http_request`-এর request ডেটা উপেক্ষা করে শর্তহীনভাবে HTML ফাইলের বিষয়বস্তু ফেরত পাঠাচ্ছি। এর মানে browser-এ _127.0.0.1:7878/something-else_ চাইলেও এই একই HTML response ফেরত
পাবেন। এই মুহূর্তে আমাদের server খুবই সীমিত এবং বেশিরভাগ web server যা করে তা করে না। আমরা request অনুযায়ী response সাজাতে চাই, আর শুধু _/_-এর জন্য সঠিক গঠনের request-এর ক্ষেত্রে HTML ফাইল ফেরত
পাঠাতে চাই।

### request যাচাই করে বেছে বেছে সাড়া দেওয়া {#validating-the-request-and-selectively-responding}

এই মুহূর্তে client যা-ই চাক, আমাদের web server ফাইলের HTML return করবে। চলুন এমন কার্যক্ষমতা যোগ করি, যা HTML ফাইল return করার আগে যাচাই করে browser _/_ চাইছে কি না, এবং browser অন্য কিছু চাইলে
একটি error return করে। এজন্য `handle_connection` বদলাতে হবে, যেমনটি Listing 21-6-এ দেখানো হয়েছে। এই নতুন কোড পাওয়া request-এর বিষয়বস্তু _/_-এর একটি request দেখতে কেমন বলে আমরা জানি তার সঙ্গে মিলিয়ে
দেখে, এবং request-গুলোকে ভিন্নভাবে সামলাতে `if` আর `else` block যোগ করে।

<Listing number="21-6" file-name="src/main.rs" caption="*/*-এর request অন্য request থেকে ভিন্নভাবে সামলানো">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-06/src/main.rs:here}}
```

</Listing>

আমরা শুধু HTTP request-এর প্রথম লাইন দেখব, তাই পুরো request একটি vector-এ না পড়ে iterator থেকে প্রথম item পেতে `next` call করছি। প্রথম `unwrap` `Option` সামলায় এবং iterator-এ কোনো item না থাকলে
প্রোগ্রাম থামায়। দ্বিতীয় `unwrap` `Result` সামলায় এবং Listing 21-2-এ যোগ করা `map`-এর ভেতরের `unwrap`-এর মতোই প্রভাব ফেলে।

এরপর `request_line` যাচাই করি, এটি _/_ path-এ একটি GET request-এর request line-এর সমান কি না। সমান হলে `if` block আমাদের HTML ফাইলের বিষয়বস্তু return করে।

`request_line` যদি _/_ path-এ GET request-এর সমান _না_ হয়, তার মানে আমরা অন্য কোনো request পেয়েছি। বাকি সব request-এর সাড়া দিতে একটু পরেই `else` block-এ কোড যোগ করব।

এখন এই কোড চালিয়ে _127.0.0.1:7878_ চান; _hello.html_-এর HTML পাওয়ার কথা। অন্য কোনো request করলে, যেমন
_127.0.0.1:7878/something-else_, Listing 21-1 আর Listing 21-2-এর কোড চালানোর সময় দেখা error-এর মতো একটি connection error পাবেন।

এবার `else` block-এ Listing 21-7-এর কোড যোগ করি, যাতে status code 404-সহ একটি response return হয়, যা জানায় request-এর বিষয়বস্তু পাওয়া যায়নি। শেষ ব্যবহারকারীকে response বোঝাতে browser-এ দেখানোর জন্য একটি
page-এর কিছু HTML-ও return করব।

<Listing number="21-7" file-name="src/main.rs" caption="*/* ছাড়া অন্য কিছু চাওয়া হলে status code 404 আর একটি error page দিয়ে সাড়া দেওয়া">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-07/src/main.rs:here}}
```

</Listing>

এখানে আমাদের response-এর status line-এ আছে status code 404 আর reason phrase
`NOT FOUND`। response-এর body হবে _404.html_ ফাইলের HTML। error page-এর জন্য _hello.html_-এর পাশে একটি _404.html_ ফাইল তৈরি করতে হবে; আবারও, যেকোনো HTML ব্যবহার করতে পারেন, অথবা Listing 21-8-এর উদাহরণ
HTML ব্যবহার করতে পারেন।

<Listing number="21-8" file-name="404.html" caption="যেকোনো 404 response-এর সঙ্গে ফেরত পাঠানোর page-এর নমুনা বিষয়বস্তু">

```html
{{#include ../listings/ch21-web-server/listing-21-07/404.html}}
```

</Listing>

এই পরিবর্তনগুলোসহ আবার আপনার server চালান। _127.0.0.1:7878_ চাইলে _hello.html_-এর বিষয়বস্তু return হওয়ার কথা, আর অন্য যেকোনো request, যেমন
_127.0.0.1:7878/foo_, _404.html_-এর error HTML return করার কথা।

<!-- Old headings. Do not remove or links may break. -->

<a id="a-touch-of-refactoring"></a>

### refactoring {#refactoring}

এই মুহূর্তে `if` আর `else` block-এ অনেক পুনরাবৃত্তি: দুটিই ফাইল পড়ছে এবং ফাইলের বিষয়বস্তু stream-এ লিখছে। পার্থক্য শুধু status line আর ফাইলের নামে। চলুন সেই পার্থক্যগুলো আলাদা `if` আর `else` লাইনে বের
করে এনে কোড আরও সংক্ষিপ্ত করি, যা status line আর ফাইলের নামের মান variable-এ বসাবে; তারপর ফাইল পড়া আর response লেখার কোডে শর্তহীনভাবে সেই variable-গুলো ব্যবহার করতে পারি। Listing 21-9-এ বড় `if` আর
`else` block বদলে দেওয়ার পরের কোড দেখানো হয়েছে।

<Listing number="21-9" file-name="src/main.rs" caption="`if` আর `else` block-কে refactor করে শুধু দুই ক্ষেত্রের ভিন্ন কোডটুকু রাখা">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-09/src/main.rs:here}}
```

</Listing>

এখন `if` আর `else` block শুধু একটি tuple-এ status line আর ফাইলের নামের উপযুক্ত মান return করে; তারপর অধ্যায় 19-এ যেমন আলোচনা করা হয়েছে, `let` statement-এ একটি pattern দিয়ে destructuring ব্যবহার করে
এই দুটি মান `status_line` আর `filename`-এ বসাই।

আগের পুনরাবৃত্ত কোড এখন `if` আর `else` block-এর বাইরে এবং `status_line` আর `filename` variable ব্যবহার করে। এতে দুই ক্ষেত্রের পার্থক্য দেখা সহজ হয়, আর ফাইল পড়া ও response লেখা কীভাবে কাজ করে তা বদলাতে
চাইলে কোড হালনাগাদ করার জায়গা মাত্র একটি। Listing 21-9-এর কোডের আচরণ Listing 21-7-এর কোডের মতোই হবে।

দারুণ! এখন আমাদের কাছে মোটামুটি 40 লাইনের Rust কোডে একটি সরল web server আছে, যা একটি request-এর উত্তরে একটি বিষয়বস্তুর page দেয় এবং বাকি সব request-এর উত্তরে একটি 404 response দেয়।

এই মুহূর্তে আমাদের server একটিমাত্র thread-এ চলে, অর্থাৎ একবারে শুধু একটি request সামলাতে পারে। কিছু ধীর request-এর অনুকরণ করে দেখা যাক এটি কীভাবে সমস্যা হতে পারে। তারপর এটি ঠিক করব, যাতে আমাদের
server একসঙ্গে একাধিক request সামলাতে পারে।
