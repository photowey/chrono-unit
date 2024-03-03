# `chrono-unit`

A `date-time` `formatter` and `time-unit` library for `Rust` `time`

## 1.`Items`

### 1.1.`Structs`

- [`formatter::DefaultDateTimeFormatter`](./src/formatter.rs)

### 1.2.`Enums`

- [`TimeUnit`](./src/lib.rs)
- [`formatter::pattern::DateTimePattern`](./src/formatter/pattern.rs)

### 1.3.`Traits`

- [`formatter::DateTimeFormatter`](./src/formatter.rs)

## 2.`Usage`

### 2.1.`TimeUnit`

#### 2.1.1.`functions`

- `to_nanos`

  - ```rust
    TimeUnit::Nanoseconds.to_nanos(1024)
    ```

  -

- `to_micros`

- `to_millis`

- `to_seconds`

- `to_minutes`

- `to_hours`

- `to_days`

- `value`

  - ```rust
    TimeUnit::value_of("Nanoseconds")
    ```

  -

### 2.2.`Formatter`

#### 2.2.1.`Builtin`

```rust
let dtf = DefaultDateTimeFormatter::builtin();
// default pattern: DateTimePattern::YyyyMmDdHhMmSs
```

#### 2.2.2.`new`

```rust
let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);
```

#### 2.2.3.`of_pattern`

```rust
let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);
let dtf = dtf.of_pattern(DateTimePattern::YyyyMmDdHhMmSs);
```

#### 2.2.4.`Format`

##### 2.2.4.1.`format_date_time_utc_default`

> - `DateTime<Utc>`
> - Default pattern

```rust
let now = "2024-03-01 02:03:04";
let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime( & ndt);

let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);

assert_eq!(
  dtf.format_date_time_utc_default(&datetime_utc),
  "2024-03-01"
);
```

##### 2.2.4.2.`format_date_time_utc`

```rust
let now = "2024-03-01 02:03:04";
let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime( & ndt);

let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);

assert_eq!(
  dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMmSs),
  "2024-03-01 02:03:04"
);
```

##### 2.2.4.3.`format_naive_date_time_utc_default`

> with timezone

```rust
let now = "2024-03-01 02:03:04";
let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);

assert_eq!(
  dtf.format_naive_date_time_utc_default(&ndt),
  "2024-03-01"
);
```

##### 2.2.4.4.`format_naive_date_time_utc`

> with timezone

```rust
let now = "2024-03-01 02:03:04";
let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);

assert_eq!(
  dtf.format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDd),
  "2024-03-01"
);
```

##### 2.2.4.5.`format_naive_date_time_default`

```rust
let now = "2024-03-01 02:03:04";
let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDdHhMmSs);

assert_eq!(
  dtf.format_naive_date_time_default(&ndt),
  "2024-03-01 02:03:04"
);
```

##### 2.2.4.6.`format_naive_date_time`

```rust
let now = "2024-03-01 02:03:04";
let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDdHhMmSs);

assert_eq!(
  dtf.format_naive_date_time(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
  "2024-03-01 02:03:04"
);
```
