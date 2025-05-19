Here's your comprehensive and expert-level design specification prompt for the tool **zedi-gen**, capturing your entire vision clearly and precisely:

---

## Expert-Level Design Specification: **zedi-gen**

### **Overview**

**zedi-gen** is a specialized, high-performance Rust-based tool designed explicitly to generate robust, realistic, and specification-adherent synthetic X12 835 healthcare claim payment/advice data sets (**X12 835 v5010**). Its primary use is rigorous testing and validation scenarios for healthcare claims processing systems. This tool will leverage deterministic yet realistic synthetic population generation, data claims creation, and precise anomaly injection capabilities.

---

### **Directory Structure**

```
zedi-gen/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── population.rs
│   ├── claims.rs
│   ├── anomalies.rs
│   ├── generator.rs
│   ├── config.rs
│   └── cli.rs
├── docs/
│   └── spec.md       # X12 835 x22181 specification
├── tests/
│   ├── integration_tests.rs
│   └── benchmarks.rs
├── windsurf/
│   └── rules.windsurf # rules derived strictly from spec.md
├── Dockerfile
└── README.md
```

---

### **Core Functional Modules**

#### **1. Configuration Management (`config.rs`)**

* Handles parsing CLI args and external config files (YAML/TOML).
* Options for controlling:

  * Synthetic population size
  * Claim complexity distribution
  * Anomaly injection percentage
  * Seed control for reproducibility

#### **2. Synthetic Population Generation (`population.rs`)**

* Generates realistic synthetic patient and provider populations:

  * Name, Address, Demographic data (age, gender, etc.)
  * Provider NPI, taxonomy codes, addresses, and specialties
* Uses statistical distributions for realistic data generation:

  * Age distribution: Census-derived
  * Geographic distribution: configurable to mirror real-world scenarios

#### **3. Claims Generation (`claims.rs`)**

* Constructs X12 835 claim records strictly adhering to the provided **spec.md**:

  * Segments: BPR, TRN, REF, DTM, N1, LX, CLP, SVC, CAS, AMT, and PLB
  * Field-level validation according to **x22181**
  * Payment scenarios: denial, partial payment, complete payment
* Deterministic control over:

  * Claim frequency
  * Line-item quantity
  * Service and adjustment codes distribution

#### **4. Anomaly Injection Engine (`anomalies.rs`)**

* Mathematically rigorous methods to introduce anomalies:

  * Percentage-based anomaly distribution
  * Anomaly categories (structural, semantic, logical, missing fields, invalid codes)
* Controlled by deterministic randomness for reproducibility
* Anomaly logging for traceability in testing

#### **5. Generator Controller (`generator.rs`)**

* Central orchestrator coordinating population generation, claims creation, and anomaly injection
* Ensures full compliance with spec.md rules
* Provides API hooks for extensibility (CLI, library)

#### **6. CLI Interface (`cli.rs`)**

* Clean, user-friendly CLI built with `clap`
* Supports two primary subcommands for generation and conformance:

  ```bash
  # Generate synthetic X12 835 claims
  zedi-gen generate --count 10000 --anomaly-rate 2.0 --data-dir data

  # Score conformance of an X12 835 file against the spec
  zedi-gen conformance path/to/claims.edi
  ```
* Command options for `generate` subcommand:

  * `-c, --count <COUNT>`: Number of claims to generate
  * `-a, --anomaly-rate <ANOMALY_RATE>`: Anomaly injection rate (0.0 to 100.0)
  * `-d, --data-dir <DATA_DIR>`: Data directory for CSV files (default: data)
  * `-f, --format <FORMAT>`: Output format [x12, json, json-pretty]
  * `-s, --seed <SEED>`: Random seed for reproducible output
  * `-o, --output <OUTPUT>`: Output file (default: stdout)
  * `-h, --help`: Print help information
  * `-V, --version`: Print version information

---

### **Integration with Windsurf IDE**

* **Windsurf Rule Definition (`rules.windsurf`)**

  ```windsurf
  RULESET X12835_Validation BASED ON docs/spec.md

  RULE ValidateSegment {
    IF segment.name NOT IN spec.segments
    THEN REJECT
  }

  RULE ValidateField {
    IF segment.field NOT MATCH spec.fieldRegex
    THEN FLAG_AS anomaly
  }

  RULE AllowableValues {
    IF segment.field.value NOT IN spec.allowedValues
    THEN FLAG_AS anomaly
  }

  RULE RequiredSegments {
    FOR segment IN spec.requiredSegments
    IF NOT EXISTS(segment IN claim)
    THEN FLAG_AS anomaly
  }
  ```
* Ensures strict alignment of the generated data to `spec.md`.

---

### **Performance & Testing**

* Extensive unit and integration tests in Rust using `cargo test`
* Benchmarking via Criterion.rs to validate performance
* High-performance synthetic generation target:

  * Millions of claims/minute throughput on commodity hardware

---

### **Dockerization**

* Compact and optimized Docker image leveraging Alpine Linux for deployment:

```Dockerfile
FROM rust:alpine as builder

WORKDIR /app
COPY . .
RUN apk add musl-dev && cargo build --release

FROM alpine:latest
COPY --from=builder /app/target/release/zedi-gen /usr/local/bin/zedi-gen

ENTRYPOINT ["zedi-gen"]
```

---

### **Implementation Plan**

* **Phase 1:** Setup and Initial Infrastructure

  * Project structure and setup
  * CLI interface and basic configuration management
* **Phase 2:** Core Data Generation Logic

  * Population and claim generation according to specification
  * Integration testing to verify compliance
* **Phase 3:** Anomaly Injection Logic

  * Develop mathematical anomaly injection system
  * Unit and integration tests to confirm precision
* **Phase 4:** Windsurf Integration

  * Develop and refine rules based strictly on `spec.md`
  * Verify integration with Windsurf IDE workflow
* **Phase 5:** Optimization and Benchmarking

  * Benchmark tests to ensure high throughput
  * Performance tuning based on identified bottlenecks
* **Phase 6:** Final Documentation and Dockerization

  * Dockerfile refinement and documentation completion
  * Final acceptance tests

---

### **Masterful System Prompt**

Here's a complete, focused, and actionable system prompt to guide implementation:

> **You are a seasoned Rust systems engineer, proficient in healthcare claims processing systems and X12 835 specifications. Implement `zedi-gen`, a specialized synthetic data generation tool strictly conforming to the `spec.md` specification located in the `docs/` directory.**
>
> **Build modularly and performantly: generate synthetic populations, realistic healthcare claims, and precise anomaly injections for extensive testing scenarios. Adhere to mathematical rigor in anomaly control. Integrate and validate using Windsurf IDE, creating rules tightly coupled to the provided specification.**
>
> **Deliver thoroughly-tested, high-performance Rust code, structured clearly as outlined, ensuring maintainability and extensibility. Final deliverable must include robust CLI tooling, thorough integration testing, performance benchmarks, detailed README documentation, and optimized Docker deployment.**

---

This design specification captures your vision of **zedi-gen** precisely, clearly structuring your development efforts toward expert-level outcomes.
