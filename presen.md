---
marp: true
theme: default
paginate: true
footer: "team TOMLer"
backgroundColor: #FFE4E1 
---

# Rust勉強会
## TOMLと`Cargo.toml`

Naoya Tezuka & Takuto Nishimura

---

# TOMLってなんですか？
## TOML = Tom’s Obvious, Minimal Language
Tomさん = このフォーマットを考案したうちの一人，Tom Preston-Wernerのこと
設定ファイルのフォーマットの1種で、JSONとかyamlとかがライバル

###  これがTOMLだ！
```toml
# This is a TOML document
title = "TOML Example"
[owner]
name = "Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00
[database]
enabled = true
ports = [ 8000, 8001, 8002 ]
```

---

# 実装されている言語
最近の言語には大体実装済み!
> TOML already has implementations in most of the most popular programming languages in use today: **C, C#, C++, Clojure, Dart, Elixir, Erlang, Go, Haskell, Java, Javascript, Lua, Objective-C, Perl, PHP, Python, Ruby, Swift, Scala**... 

<br>

詳しくは[ここ](https://github.com/toml-lang/toml/wiki)


---
# TOMLの構文
## テーブル(の配列)
```toml
[[products]]            # {
name = "Hammer"         #   "products": [
sku = 738594937         #     { "name": "Hammer", "sku": "738594937"},    
                        #       {},
[[products]]            #       { "name": "Nail", "sku": 284758393, "color": "gray" },
                        #   ]
[[products]]            # }
name = "Nail"
color = "gray"
```
---

# TOMLの構文
コメント
```toml
# This is a TOML comment 
```
文字列
```toml
str1 = "Please call me \"TOM\"\n" 
str2 = """
    You can use \   
    multi-line strings. \
    """
str3 = 'single quotes mean "no escape"'
```
---

# TOMLの構文
## 数(bin, oct, hexもあるよ)
```toml
int1 = 32
float2 = -3.14
inf3 = +inf
not4 = nan
bool5 = true
```
## 日付と時間
```toml
# 現地の日付
ld1 = 1979-05-27
# 現地の時間
lt2 = 21:59:44
```

---

# YAML vs JSON 
# vs TOML 



---

# TOMLの良いところ1
## 仕様書が小さい
[YAML](https://yaml.org/spec/1.2/spec.html)：28スクロール！
[JSON](https://datatracker.ietf.org/doc/html/rfc8259)：6スクロール
[TOML](https://toml.io/en/v1.0.0#objectives)：7スクロール

実際にはTOMLの仕様書は殆どがサンプルコード

→ 本当に**言語仕様が小さい**!!

---

# TOMLの良いところ2
## コメントが書きやすい

YAML：# で簡単(アレ？)
JSON：かけません！！
TOML：# で簡単

---

# TOMLの良いところ3

## 人間にとって読みやすい
```yaml
- YAML: 
 - インデント: 変えないとねえ
```
```json
"JSON": {"クオーテーション多すぎ！": {"カッコも多すぎ": "ぴえん"}}
```
```toml
TOML = "読みやすい"
```

---

# TOMLの良いところ4
## パースしやすい(らしい)

YAML：めっちゃむずい
JSON：簡単
TOML：簡単

---
# TOMLのまとめ


---

# `Cargo.toml`