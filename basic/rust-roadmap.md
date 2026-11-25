# Rust Expert Roadmap - 30 Hours/Week (Accelerated)

**Time Commitment:** 6 hours/day × 5 days/week = 30 hours/week  
**Estimated Duration:** 4-5 months to expert level

---

## Phase 1: Foundation & Ownership Mastery (Weeks 1-3)

**30 hours/week breakdown:** 10h reading/theory, 20h coding

### Week 1: Rapid Refresher
- **Mon-Tue:** Rust Book Ch 1-6 (ownership, references, slices)
- **Wed:** Rust Book Ch 7-8 (modules, collections)
- **Thu-Fri:** Complete ALL Rustlings (80+ exercises)
- **Weekend Project:** CLI tool with clap (file searcher or calculator)
- **Deliverable:** Working CLI with proper error handling

### Week 2: Structs, Enums & Pattern Matching
- **Mon:** Rust Book Ch 6 (enums), Ch 9 (error handling)
- **Tue:** Implement 5 different error types using `thiserror`
- **Wed-Thu:** Pattern matching mastery - exhaustive matching, guards, bindings
- **Fri:** Build a state machine using enums
- **Weekend Project:** JSON parser or config file reader
- **Deliverable:** Parser with custom error types

### Week 3: Collections & Iterators
- **Mon:** Deep dive into Vec, HashMap, BTreeMap internals
- **Tue:** Iterator trait - map, filter, fold, collect patterns
- **Wed:** Build custom Iterator implementations
- **Thu:** Exercism Rust track (10+ medium problems)
- **Fri:** Performance comparison: iterator vs loop
- **Weekend Project:** Data processing pipeline (CSV → analysis → report)
- **Checkpoint:** Write 300+ line programs confidently

---

## Phase 2: Generics, Traits & Lifetimes (Weeks 4-6)

### Week 4: Generics & Trait Bounds
- **Mon:** Generic functions, structs, methods
- **Tue:** Trait bounds and where clauses
- **Wed:** Associated types vs generic parameters
- **Thu:** Build generic LinkedList and HashMap from scratch
- **Fri:** Compare your implementation with std library
- **Weekend Project:** Generic caching library with TTL support

### Week 5: Lifetimes Deep Dive
- **Mon:** Lifetime elision rules and annotations
- **Tue:** Lifetime in structs and self-referential structs
- **Wed:** The 'static lifetime and trait objects
- **Thu:** Advanced patterns: lifetime bounds, higher-ranked traits
- **Fri:** Owning_ref and rental crates (understand their need)
- **Weekend Project:** Arena allocator or graph structure with references

### Week 6: Advanced Traits & Object Safety
- **Mon:** Trait objects (dyn Trait) and object safety
- **Tue:** Operator overloading (Add, Deref, Drop)
- **Wed:** Extension traits and blanket implementations
- **Thu:** Build plugin architecture with trait objects
- **Fri:** Design patterns: visitor, strategy, builder
- **Weekend Project:** Mini-framework with plugin system
- **Checkpoint:** Comfortable with lifetime annotations, trait objects

---

## Phase 3: Systems Programming & Concurrency (Weeks 7-10)

### Week 7: Unsafe Rust & FFI
- **Mon:** Raw pointers, unsafe blocks, undefined behavior
- **Mon-Tue:** Rustonomicon Ch 1-4 (must read!)
- **Wed:** FFI basics - calling C from Rust
- **Thu:** Bindgen for automatic bindings
- **Fri:** Build wrapper for a small C library (e.g., sqlite3 or zlib)
- **Weekend Project:** Memory-mapped file reader using unsafe

### Week 8: Smart Pointers & Interior Mutability
- **Mon:** Box, Rc, Arc - when to use each
- **Tue:** RefCell, Mutex, RwLock - interior mutability patterns
- **Wed:** Weak references and memory leaks
- **Thu:** Build thread-safe data structures
- **Fri:** Compare sync vs async patterns
- **Weekend Project:** Thread-safe LRU cache

### Week 9: Concurrent Programming
- **Mon:** Threads, join handles, scoped threads
- **Tue:** Message passing with channels (mpsc, broadcast)
- **Wed:** Shared state: Arc<Mutex<T>> patterns
- **Thu:** Thread pools and work stealing
- **Fri:** Crossbeam for advanced concurrency
- **Weekend Project:** Multi-threaded web crawler with rate limiting

### Week 10: Async/Await & Tokio
- **Mon:** Futures, executors, async/await syntax
- **Tue:** Tokio runtime, tasks, spawning
- **Wed:** Async channels, select!, join!
- **Thu:** Async traits and Pin/Unpin
- **Fri:** Build async HTTP client with reqwest/hyper
- **Weekend Project:** Real-time chat server with WebSockets
- **Checkpoint:** Build concurrent applications confidently

---

## Phase 4: Advanced Patterns & Specialization (Weeks 11-16)

### Week 11: Declarative Macros
- **Mon-Tue:** macro_rules! syntax, patterns, repetitions
- **Wed:** Hygiene and scoping in macros
- **Thu-Fri:** Build 3+ useful macros (vec!, hashmap! clones)
- **Weekend Project:** DSL for configuration or testing

### Week 12: Procedural Macros
- **Mon:** Function-like proc macros
- **Tue-Wed:** Derive macros (CustomDebug, Serialize clones)
- **Thu:** Attribute macros
- **Fri:** Token manipulation with proc-macro2
- **Weekend Project:** Custom derive for a domain-specific need

### Week 13-14: Pick Your Specialization

**Option A - Web Backend:**
- Axum or Actix-web framework
- Database: SQLx or Diesel
- Authentication: JWT, sessions
- Testing: integration tests with testcontainers
- **Project:** Production-ready REST API with auth, tests, docs

**Option B - Systems/Embedded:**
- Bare metal programming with no_std
- Embassy framework for async embedded
- Memory-mapped I/O, interrupts
- **Project:** Firmware for sensor reading with wireless transmission

**Option C - WebAssembly:**
- wasm-bindgen, web-sys
- Yew or Leptos frontend framework
- JS interop
- **Project:** Interactive web app with complex state

**Option D - Game Development:**
- Bevy ECS architecture
- Rendering, physics, audio
- **Project:** 2D game with complex mechanics

**Option E - CLI/Systems Tools:**
- ratatui for TUI applications
- crossterm for cross-platform terminal
- **Project:** TUI file manager or system monitor

### Week 15: Testing & Quality
- **Mon:** Unit testing best practices
- **Tue:** Property-based testing with proptest
- **Wed:** Fuzzing with cargo-fuzz
- **Thu:** Mutation testing
- **Fri:** CI/CD for Rust projects
- **Weekend:** Achieve 90%+ coverage on specialization project

### Week 16: Profiling & Optimization
- **Mon:** Benchmarking with Criterion
- **Tue:** CPU profiling with perf/flamegraph
- **Wed:** Memory profiling with heaptrack
- **Thu:** Compiler optimizations and codegen
- **Fri:** Unsafe optimizations (when justified)
- **Weekend:** Optimize specialization project (10x improvement)

---

## Phase 5: Expert Level & Ecosystem (Weeks 17-20)

### Week 17: Read the Source
- **Mon-Wed:** Read 3+ standard library modules (Vec, HashMap, Iterator)
- **Thu-Fri:** Study 2 popular crates in your specialization
- **Weekend:** Write detailed analysis blog post

### Week 18: Open Source Contribution
- **Mon:** Find 3-5 projects, read issues
- **Tue-Thu:** Fix 2-3 "good first issues"
- **Fri:** Tackle a medium-complexity issue
- **Weekend:** Write contribution guide based on your experience

### Week 19: Build & Publish a Crate
- **Mon:** Identify ecosystem gap
- **Tue-Wed:** Design public API (focus on ergonomics)
- **Thu-Fri:** Implement with comprehensive tests
- **Weekend:** Publish to crates.io, write README

### Week 20: Teach & Lead
- **Mon:** Write technical blog post on advanced topic
- **Tue:** Create tutorial or video
- **Wed:** Mentor someone on r/rust or Discord
- **Thu-Fri:** Review others' Rust code
- **Weekend:** Give presentation or workshop

---

## Weekly Schedule Template (30 hours)

| Day | Morning (3h) | Afternoon (3h) |
|-----|--------------|----------------|
| Mon | Read theory/book | Code exercises |
| Tue | Read theory/book | Code exercises |
| Wed | Read theory/book | Code exercises |
| Thu | Deep dive topic | Build project component |
| Fri | Code review/refactor | Build project component |
| Sat | Full project day | Full project day |

---

## Daily Structure (6 hours)

**Hours 1-2: Theory**
- Read book chapters
- Watch technical videos (Jon Gjengset, Ryan Levick)
- Study documentation

**Hours 3-4: Practice**
- Complete exercises
- Solve algorithm problems
- Experiment with patterns

**Hours 5-6: Build**
- Work on weekly project
- Contribute to open source
- Write documentation/tests

---

## Accelerated Success Metrics

**By Week 6 (1.5 months):**
- 1000+ lines of Rust written
- Can debug lifetime errors independently
- Built 3 complete CLI tools

**By Week 10 (2.5 months):**
- Comfortable with unsafe code
- Built concurrent applications
- Understand async ecosystem

**By Week 16 (4 months):**
- Production-ready project in specialization
- Published crate or significant PR merged
- Can mentor others

**By Week 20 (5 months):**
- Recognized in Rust community
- Expert-level code quality
- Ready for senior Rust engineer roles

---

## Essential Resources

### Books (in order):
1. **"The Rust Programming Language"** (The Book) - Weeks 1-3
2. **"Programming Rust"** by Blandy & Orendorff - Weeks 4-6
3. **"Rust for Rustaceans"** by Jon Gjengset - Weeks 7-10
4. **"Rust Atomics and Locks"** by Mara Bos - Weeks 9-10
5. **"The Rustonomicon"** - Reference throughout

### Video Channels:
- **Jon Gjengset** - Low-level, async, unsafe
- **Let's Get Rusty** - Beginner-friendly
- **Ryan Levick** - Systems programming
- **No Boilerplate** - Architecture patterns

### Practice Platforms:
- **Rustlings** - Week 1 (complete all)
- **Exercism** - Ongoing practice
- **LeetCode** - Weekly challenges
- **Codewars** - Daily kata
- **Advent of Code** - Year-end challenge

### Community Engagement:
- Read **This Week in Rust** weekly
- Participate in **r/rust** discussions
- Join **Rust Discord** #beginners and #help channels
- Attend **Rust meetups** (virtual or local)

---

## Specialized Resources by Track

### Web Backend:
- [Axum](https://docs.rs/axum/) or [Actix-web](https://actix.rs/)
- [SQLx](https://github.com/launchbadge/sqlx) or [Diesel](https://diesel.rs/)
- [Tower](https://docs.rs/tower/) for middleware

### Systems/Embedded:
- [Embassy](https://embassy.dev/) for async embedded
- [RTIC](https://rtic.rs/) for real-time
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)

### WebAssembly:
- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)
- [Yew](https://yew.rs/) or [Leptos](https://leptos.dev/)
- [The Rust and WebAssembly Book](https://rustwasm.github.io/book/)

### Game Development:
- [Bevy Engine](https://bevyengine.org/)
- [The Bevy Book](https://bevyengine.org/learn/book/introduction/)

### CLI Tools:
- [clap](https://docs.rs/clap/) for argument parsing
- [ratatui](https://github.com/ratatui/ratatui) for TUI
- [crossterm](https://docs.rs/crossterm/) for terminal control

---

## Tips for Success

1. **Code every day** - Consistency beats intensity
2. **Read error messages carefully** - The compiler is your teacher
3. **Build projects, not just exercises** - Real learning comes from solving real problems
4. **Read others' code** - Study popular crates on crates.io
5. **Teach what you learn** - Explaining solidifies understanding
6. **Don't skip unsafe** - Even if you don't use it, you need to understand it
7. **Embrace the borrow checker** - Work with it, not against it
8. **Join the community** - Rust has one of the most welcoming communities

---

## Questions to Consider

1. Which specialization interests you most? (Web, Systems, WASM, Game Dev, CLI)
2. Do you have a specific project in mind?
3. Preferred learning style? (Videos, books, hands-on, or mix?)
4. Do you want weekly check-ins or milestones?

---

*Created for 30 hours/week intensive study. Adjust as needed for your schedule.*
*Good luck on your Rust journey!*
