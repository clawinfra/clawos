# ClawOS Roadmap

ClawOS development is **parked** while we focus on [EvoClaw](https://github.com/clawinfra/evoclaw). This roadmap outlines the path forward when development resumes.

---

## Current Status: PARKED 🅿️

**Why parked?**
1. **EvoClaw needs to mature first** — prove the agent evolution model works
2. **Small team, big vision** — need to focus on one thing at a time
3. **Community validation** — need real users to validate the need for a dedicated OS

**What we're doing instead:**
- Building EvoClaw framework (agent runtime, genomes, evolution)
- Testing EvoClaw on existing OSes (Linux, macOS, Windows)
- Gathering community feedback and contributors
- Validating companion agent use case (Pi Zero 2W with standard Linux)

**When to resume:**
- EvoClaw reaches **50+ stars** (proves interest)
- Companion use case **validated** (users running agents on Pi)
- Contributors interested in **embedded Linux or kernel work**

---

## Phase 1: ClawOS Lite

**Goal:** Minimal Linux distro optimized for EvoClaw agents.

**Target hardware:** Raspberry Pi Zero 2W, Pi 3/4/5

**Timeline:** When EvoClaw has 50+ stars

### Milestones

#### 1.1: Buildroot Prototype
**Deliverables:**
- Buildroot configuration for Pi Zero 2W
- Custom Linux kernel (minimal config, <5 MB)
- Busybox userland
- Simple init system (replaces systemd)
- Total image size: <50 MB (unoptimized prototype)

**Success criteria:**
- Boots to shell in <5 seconds
- 512 MB image (SD card ready)

**Estimated effort:** 2-3 weeks (1 person with Buildroot experience)

---

#### 1.2: EvoClaw Integration
**Deliverables:**
- Pre-install EvoClaw agent runtime (Node.js or native)
- Pre-install MQTT broker (Mosquitto)
- Init system starts EvoClaw agent on boot
- Agent loads default genome from `/var/agent/genomes/`

**Success criteria:**
- Boot to agent-ready in <3 seconds
- Agent can run simple genome (e.g., echo "hello world")

**Estimated effort:** 1-2 weeks

---

#### 1.3: Audio Stack
**Deliverables:**
- ALSA drivers for USB and I2S audio
- PipeWire (lightweight audio routing)
- Test with USB microphone + 3.5mm speaker
- Test with I2S microphone + I2S DAC

**Success criteria:**
- Agent can record audio from mic
- Agent can play audio to speaker
- Latency <50ms (mic to speaker)

**Estimated effort:** 2-3 weeks (audio on embedded Linux is hard)

---

#### 1.4: OTA Update System
**Deliverables:**
- A/B partition scheme (rootfs_a + rootfs_b)
- U-Boot configuration for A/B switching
- OTA update daemon (downloads new image, writes to inactive partition)
- Rollback mechanism (if new image fails to boot, revert to old)

**Success criteria:**
- Download 30 MB image over WiFi (~5 minutes on slow network)
- Atomically switch partitions on reboot
- Rollback if agent fails to start 3 times

**Estimated effort:** 2-3 weeks

---

#### 1.5: Image Optimization
**Deliverables:**
- Compress rootfs with SquashFS
- Remove unnecessary kernel modules
- Strip binaries (remove debug symbols)
- Optimize kernel config (disable unused subsystems)

**Success criteria:**
- Total image size: <30 MB (compressed)
- Boot time: <2 seconds (power-on to agent-ready)
- RAM usage: <150 MB (OS + agent idle)

**Estimated effort:** 1-2 weeks

---

#### 1.6: Documentation & Release
**Deliverables:**
- Installation guide (flash SD card, boot Pi)
- Configuration guide (WiFi setup, agent config)
- Troubleshooting guide (common issues)
- Developer guide (how to build custom images)

**Success criteria:**
- Non-technical user can flash SD card and boot agent in <15 minutes
- Developer can build custom image from source

**Estimated effort:** 1 week

---

### Phase 1 Summary

**Total estimated effort:** 10-15 weeks (2.5-4 months)

**Deliverables:**
- ClawOS Lite 1.0 release
- SD card image for Pi Zero 2W (download and flash)
- Documentation (install, configure, develop)
- OTA update system (agents and OS can update)

**Success metrics:**
- 100+ users running ClawOS Lite on Pi
- 5+ contributors to repo
- <5 critical bugs in first 3 months

---

## Phase 2: Audio & Companion Validation

**Goal:** Validate companion agent use case (always-on voice assistant).

**Target hardware:** Pi Zero 2W with microphone + speaker

**Timeline:** When EvoClaw companion use case is validated

### Milestones

#### 2.1: STT/TTS Pipeline Integration
**Deliverables:**
- Pre-install Whisper.cpp (on-device STT)
- Pre-install Piper (on-device TTS)
- Audio pipeline: mic → VAD → STT → agent → TTS → speaker
- Cloud STT/TTS fallback (if on-device fails)

**Success criteria:**
- Agent can transcribe speech in <1 second (cloud) or <3 seconds (on-device)
- Agent can synthesize speech in <500ms (cloud) or <2 seconds (on-device)
- Total latency: <5 seconds (wake word → response spoken)

**Estimated effort:** 3-4 weeks

---

#### 2.2: Wake Word Detection
**Deliverables:**
- Pre-install Porcupine (on-device wake word detection)
- Custom wake word: "Hey Claw" (or user-configurable)
- Low-power mode: wake word detection runs, agent sleeps

**Success criteria:**
- Wake word detection <500ms latency
- False positive rate <1% (doesn't wake on random noise)
- Low CPU usage (<5% when idle)

**Estimated effort:** 2-3 weeks

---

#### 2.3: Companion Agent Genome
**Deliverables:**
- Default companion genome (EvoClaw persona)
- Conversational AI (GPT-4 or local LLM)
- Memory persistence (remember past conversations)
- Skills: weather, timers, reminders, smart home control

**Success criteria:**
- Agent responds naturally to voice commands
- Agent remembers user preferences
- Agent can control smart home devices (via MQTT)

**Estimated effort:** 4-6 weeks (mostly EvoClaw work, not ClawOS)

---

#### 2.4: Reference Hardware Design
**Deliverables:**
- BOM (bill of materials) for companion device
- 3D-printable case design (STL files)
- Wiring diagram (USB mic, 3.5mm speaker, power)
- Assembly guide (step-by-step instructions)

**Success criteria:**
- Total cost <$50
- Non-technical user can assemble in <30 minutes

**Estimated effort:** 2-3 weeks (hardware design + documentation)

---

### Phase 2 Summary

**Total estimated effort:** 11-16 weeks (3-4 months)

**Deliverables:**
- ClawOS Lite 2.0 with audio stack
- Reference companion device design
- Working voice assistant (wake word → STT → agent → TTS)

**Success metrics:**
- 500+ users running companion agents
- 10+ community-designed cases/hardware mods
- <10% users revert to standard Raspbian (i.e., ClawOS is good enough)

---

## Phase 3: ClawOS Core Design

**Goal:** Design custom microkernel OS for ultra-low-power devices.

**Target hardware:** ESP32-S3 (and future ARM Cortex-M devices)

**Timeline:** When EvoClaw has 100+ contributors

### Milestones

#### 3.1: Kernel Architecture Design
**Deliverables:**
- Kernel design document (seL4-based or custom)
- Agent runtime design (userspace service)
- IPC mechanism design (shared memory + capabilities)
- Boot flow design (bootloader → kernel → runtime → agent)

**Success criteria:**
- Peer review by kernel developers
- Design supports <500ms boot time
- Design supports <5 MB total image size

**Estimated effort:** 4-6 weeks (design + review)

---

#### 3.2: seL4 Evaluation
**Deliverables:**
- Port seL4 to ESP32-S3 (if not already done)
- Benchmark seL4 on ESP32-S3 (boot time, IPC latency, memory usage)
- Decide: seL4 or custom kernel?

**Success criteria:**
- seL4 boots on ESP32-S3 in <100ms
- IPC latency <10μs
- Total kernel size <500 KB

**Estimated effort:** 6-8 weeks (kernel porting is hard)

---

#### 3.3: Agent Runtime Service
**Deliverables:**
- Agent runtime as userspace service (in Rust or C)
- Genome loader (load WASM or native binaries)
- IPC setup (grant capabilities to agents)
- Fault recovery (restart crashed agents)

**Success criteria:**
- Runtime boots in <200ms
- Can load and run simple agent (echo genome)
- Agent isolation verified (one agent can't read another's memory)

**Estimated effort:** 6-8 weeks

---

#### 3.4: Minimal Drivers
**Deliverables:**
- WiFi driver (userspace, talks to ESP32 WiFi stack)
- I2S audio driver (userspace)
- GPIO driver (userspace)
- Storage driver (SPI flash)

**Success criteria:**
- WiFi connects to network in <2 seconds
- Audio latency <50ms
- Drivers are <500 KB total

**Estimated effort:** 6-8 weeks (driver development is tedious)

---

#### 3.5: Prototype & Benchmark
**Deliverables:**
- ClawOS Core prototype on ESP32-S3
- Boot to agent-ready in <500ms
- Total image size <5 MB
- Agent can run simple genome (audio echo, blink LED)

**Success criteria:**
- Meets boot time goal (<500ms)
- Meets image size goal (<5 MB)
- Stable (no crashes in 24-hour test)

**Estimated effort:** 4-6 weeks (integration + testing)

---

### Phase 3 Summary

**Total estimated effort:** 26-36 weeks (6-9 months)

**Deliverables:**
- ClawOS Core prototype on ESP32-S3
- Design documents and architecture
- Benchmark results (boot time, latency, memory)

**Success metrics:**
- Proof-of-concept works (boots, runs agent, stable)
- Community agrees design is sound
- 3+ contributors working on kernel/drivers

---

## Phase 4: ClawOS Core Implementation

**Goal:** Production-ready ClawOS Core for ESP32-S3 and ARM devices.

**Timeline:** 2027+ (depends on Phase 3 success)

### Milestones

*(High-level, details TBD based on Phase 3 learnings)*

#### 4.1: Production Kernel
- Formal verification (if using seL4)
- Security audit
- Performance tuning
- Multi-core support (SMP or AMP)

#### 4.2: Full Driver Support
- Bluetooth LE driver
- Camera driver (for ESP32-CAM)
- Sensor drivers (I2C/SPI)
- OTA update driver

#### 4.3: Multi-Agent Support
- Agent-to-agent IPC (shared memory)
- Agent scheduling (real-time priorities)
- Agent isolation (memory protection)

#### 4.4: ClawOS Core 1.0 Release
- SD card image for ESP32-S3
- Documentation (install, develop, deploy)
- OTA update system
- Reference hardware designs (wearable, sensor node)

---

### Phase 4 Summary

**Total estimated effort:** 52-78 weeks (12-18 months)

**Deliverables:**
- ClawOS Core 1.0 release
- Support for ESP32-S3 + ARM Cortex-M devices
- Multi-agent coordination primitives
- OTA update system

**Success metrics:**
- 1,000+ devices running ClawOS Core
- 10+ commercial products using ClawOS
- <1% critical bug rate

---

## Phase 5: ClawOS Full (Future Vision)

**Goal:** Distributed agent mesh + ClawChain integration.

**Timeline:** 2027+ (speculative)

### Potential Features

- **Distributed agent discovery:** mDNS, gossip protocols
- **ClawChain client:** On-device blockchain node (light client)
- **Agent-to-agent trust:** Verify agent genomes via ClawChain
- **Hardware attestation:** TPM/TrustZone for secure boot
- **Agent mesh networking:** Agents coordinate across devices
- **Swarm intelligence:** Multi-agent coordination primitives

### Targets

- IoT swarms (sensor networks)
- Robot swarms (coordinated robotics)
- Edge AI clusters (distributed inference)

---

## Alternative Paths

### If EvoClaw pivots:
- ClawOS could support other agent frameworks (AutoGPT, LangChain, etc.)
- Focus on "minimal OS for edge AI" instead of "OS for EvoClaw"

### If community wants faster progress:
- Skip ClawOS Core (Phase 3-4), focus only on ClawOS Lite
- Partner with existing embedded Linux vendors (Yocto Project, Buildroot community)

### If resources become available:
- Hire embedded Linux expert (speed up Phase 1-2)
- Hire kernel developer (speed up Phase 3-4)
- Fund hardware manufacturing (sell pre-built companion devices)

---

## How to Help

ClawOS is parked, but you can prepare for future development:

### Now (while parked):
- **Use EvoClaw:** Run agents on Pi Zero 2W with standard Linux, report issues
- **Design hardware:** Create 3D-printable cases, circuit boards, BOMs
- **Write docs:** Tutorials for running agents on embedded devices
- **Experiment:** Try Buildroot, Yocto, seL4 — share learnings

### When Phase 1 starts:
- **Test images:** Flash ClawOS Lite, report bugs
- **Contribute code:** Custom init system, OTA updater, drivers
- **Write genomes:** Create companion agent personalities
- **Spread the word:** Blog posts, YouTube videos, conference talks

### When Phase 3 starts:
- **Port seL4:** Help port kernel to new architectures
- **Write drivers:** WiFi, audio, sensors for ClawOS Core
- **Design hardware:** Custom boards optimized for ClawOS

---

## Success Metrics (Long-Term)

**Phase 1 (ClawOS Lite):**
- 1,000+ users running on Pi
- 10+ contributors
- 5+ commercial products

**Phase 2 (Companion Validation):**
- 5,000+ companion devices in the wild
- 50+ community hardware designs
- Major media coverage (Hacker News front page, tech blogs)

**Phase 3-4 (ClawOS Core):**
- 10,000+ devices (ESP32, ARM Cortex-M)
- 100+ contributors
- 50+ commercial products

**Phase 5 (ClawOS Full):**
- 100,000+ devices (agent swarms, IoT, edge AI)
- Standard OS for agent-native hardware
- Academic research (papers, theses)

---

## Questions?

**Timelines:** All estimates assume 1-2 full-time developers. Community contributions can accelerate.

**Dependencies:** ClawOS depends on EvoClaw maturity. If EvoClaw stalls, ClawOS stalls.

**Funding:** All phases assume volunteer/open-source effort. Funding could 2-3x speed.

**Open a discussion:** [github.com/clawinfra/clawos/discussions](https://github.com/clawinfra/clawos/discussions)

---

**Status: PARKED.** Focus on [EvoClaw](https://github.com/clawinfra/evoclaw) first. ClawOS will resume when the time is right.

**Be water. Flow when ready.** 🌊
