# ClawOS Vision

> **"Be water, my agent"** — adapts to any container, but flows fastest in the right channel.

---

## Why a New OS for Agents?

The future of computing isn't apps — it's **agents**. Autonomous, evolving AI agents that adapt to your needs, learn over time, and coordinate with other agents to solve complex problems.

But today's operating systems were designed for a different era:
- Linux, Windows, macOS were built for **users running applications**
- Containers (Docker, etc.) were built for **microservices and web apps**
- IoT/embedded OSes focus on **sensors and hardware**, not intelligence

**None were designed with AI agents as first-class citizens.**

ClawOS reimagines the OS from the ground up for the **age of evolving agents**.

---

## The Problems with General-Purpose OSes

Running [EvoClaw](https://github.com/clawinfra/evoclaw) agents on traditional operating systems works, but it's like running a Formula 1 car on a dirt road. It works, but there's friction everywhere.

### 1. **Bloat**
- General-purpose Linux distros: **2-4 GB** installed
- macOS/Windows: **20-50 GB**
- **Most of it is unused** — desktop environments, GUI libraries, legacy drivers, dozens of unused services

An EvoClaw agent needs:
- A runtime (Node.js, Python, or native binary)
- Network stack
- Audio I/O (for companion agents)
- Persistent storage
- Maybe MQTT or WebSocket broker

That's it. Yet it runs on **gigabytes of OS bloat**.

### 2. **Boot Time**
- Raspberry Pi running Raspbian: **15-30 seconds** to SSH-ready
- Typical embedded Linux: **5-15 seconds**
- Even "minimal" distros: **3-5 seconds**

For an **always-on companion agent**, this is fine — boot once, run forever.

But for:
- **Edge agents** that wake on sensor triggers
- **Battery-powered devices** that sleep most of the time
- **Swarm agents** that need instant coordination

Every second of boot time is wasted energy and latency.

### 3. **Security Model Mismatch**
Traditional OS security is built around:
- **Users and permissions** (UID/GID, file permissions)
- **Process isolation** (virtual memory, namespaces)
- **Network firewalls** (iptables, firewalld)

But for AI agents, we need:
- **Agent attestation** — prove this agent is running the correct genome
- **Genome integrity** — verify agent code hasn't been tampered with
- **Agent-to-agent trust** — secure communication between agents
- **Hardware-backed secrets** — keys stored in secure enclaves (TPM, TEE)

### 4. **No Agent Primitives**
On Linux/macOS/Windows, an agent is just a **process**. The OS has no concept of:
- **Agent identity** (beyond PID)
- **Agent genome** (the code that defines behavior)
- **Agent evolution** (updating genomes over-the-air)
- **Agent coordination** (beyond basic IPC)

Agents end up building these primitives **in userspace**, reinventing the wheel every time.

### 5. **Resource Waste**
A typical EvoClaw agent might use:
- **50-150 MB RAM** (Node.js runtime + agent code)
- **100-500 MB storage** (runtime + dependencies)
- **Minimal CPU** when idle

But the OS underneath consumes:
- **200-500 MB RAM** (kernel, systemd, services)
- **2-4 GB storage** (full distro)
- **5-10% CPU** (background services)

For a **Raspberry Pi Zero 2W** with 512 MB RAM, the OS is eating half your memory before the agent even starts.

---

## ClawOS Design Principles

ClawOS is built around eight core principles:

### 1. **Agent as First-Class Citizen**

In ClawOS, an agent isn't just a process — it's the **primary abstraction**.

The OS knows:
- **Agent identity** (UUID, public key)
- **Agent genome** (code hash, version)
- **Agent state** (running, sleeping, evolving)
- **Agent capabilities** (sensors, actuators, network)

The init system starts **agents**, not services. The scheduler prioritizes **agent tasks**, not processes. The filesystem is organized around **agent data**, not users.

### 2. **Boot in <500ms**

Every millisecond counts for edge and battery-powered agents.

ClawOS targets:
- **ClawOS Lite (Linux-based):** <2 seconds to agent-ready
- **ClawOS Core (microkernel):** <500ms to agent-ready

How?
- Minimal kernel (no modules, no unnecessary drivers)
- No initramfs (direct boot)
- Custom init system (no systemd overhead)
- Agent starts **as init** (PID 1) or immediately after

### 3. **Total Image <30MB (Lite) / <5MB (Core)**

Small images mean:
- **Faster OTA updates** (download 30MB vs 2GB)
- **Less storage needed** (cheap embedded flash)
- **Better security** (smaller attack surface)
- **Easier auditing** (can actually read all the code)

ClawOS Lite (~30MB):
- Custom Linux kernel
- Busybox userland
- EvoClaw runtime
- Audio stack (ALSA/PipeWire)
- MQTT broker (Mosquitto)

ClawOS Core (~5MB):
- Custom microkernel
- Agent runtime (built-in)
- Minimal drivers
- Zero userland bloat

### 4. **Secure Boot + Agent Attestation**

Trust starts at boot. ClawOS provides:

**Hardware root of trust:**
- UEFI Secure Boot (x86/ARM)
- TPM 2.0 or discrete TPM chips
- ARM TrustZone / Intel SGX where available

**Verified boot chain:**
- Bootloader verifies kernel signature
- Kernel verifies init/agent signature
- Agent verifies genome integrity

**Agent attestation:**
- Agents report genome hash to ClawChain
- Other agents can verify identity before trusting
- Remote attestation for cloud coordination

### 5. **Zero-Copy IPC Between Agents**

When agents coordinate, every memory copy is waste.

Traditional IPC:
```
Agent A → kernel buffer → Agent B  (2 copies)
```

ClawOS shared memory IPC:
```
Agent A → shared memory ← Agent B  (0 copies)
```

ClawOS Core provides:
- **Capability-based shared memory** (seL4 style)
- **Message passing** for small messages
- **Shared buffers** for large data (audio, images)
- **Agent-to-agent channels** with OS-enforced access control

### 6. **Built-in Evolution Runtime**

Agents evolve. The OS should support this natively.

ClawOS provides:
- **Genome versioning** (track which genome is running)
- **Hot-swapping** (update agent code without reboot)
- **Rollback** (revert to previous genome if new one fails)
- **A/B partitions** (safe OTA updates)

Evolution isn't an afterthought — it's **built into the OS**.

### 7. **Over-the-Air Genome Updates**

ClawOS includes an OTA update system for:
- **Agent genomes** (the AI/agent code)
- **OS updates** (kernel, init system)
- **Firmware** (bootloader, drivers)

Updates are:
- **Atomic** (A/B partitions, never brick the device)
- **Verified** (signed by developer, checked by bootloader)
- **Efficient** (delta updates, only send diffs)
- **Resilient** (automatic rollback on failure)

An agent can evolve **in the field** without human intervention.

### 8. **Hardware Abstraction Layer for Sensors/Actuators**

Agents interact with the physical world. ClawOS makes this simple.

**HAL design:**
```
Agent → ClawOS HAL → Driver → Hardware
```

**Unified interface for:**
- **Microphones** (audio input)
- **Speakers** (audio output)
- **Cameras** (image/video input)
- **Displays** (visual output)
- **LEDs** (status indicators)
- **Buttons** (physical input)
- **GPIO** (custom sensors/actuators)
- **I2C/SPI/UART** (raw device access)

Agents use a **simple API** that works across hardware platforms. Write once, run on Pi, ESP32, x86, etc.

---

## Target Hardware

ClawOS is designed to run on **resource-constrained edge devices**, not data center servers.

### Primary Targets

**Raspberry Pi Zero 2W:**
- **Why:** Cheap ($15), quad-core, 512MB RAM, GPIO, camera/audio support
- **ClawOS Lite:** Perfect fit for minimal Linux distro
- **Use case:** Companion agents, home automation, edge AI

**ESP32-S3:**
- **Why:** Tiny ($5-10), dual-core, WiFi/BLE, 512KB RAM, 8MB flash
- **ClawOS Core:** Future microkernel target
- **Use case:** Sensor agents, wearables, battery-powered devices

### Supported Hardware

- Raspberry Pi 3/4/5 (more power, still edge devices)
- BeagleBone Black/AI (industrial edge)
- NVIDIA Jetson Nano (edge AI with GPU)

### Community Targets

- x86 embedded boards (Intel NUC, Fitlet)
- RISC-V boards (future, for open hardware)
- Custom silicon (when agent ASICs exist)

See [HARDWARE.md](HARDWARE.md) for detailed specs.

---

## Architecture Overview

ClawOS comes in three phases, each more minimal than the last.

### ClawOS Lite (Phase 1)

A **minimal Linux distribution** built with Buildroot or Yocto.

**Base:**
- Custom Linux kernel (no modules, only needed drivers)
- Busybox userland (minimal GNU tools)
- Custom init system (agent-first, not systemd)

**Pre-installed:**
- EvoClaw agent runtime
- MQTT broker (Mosquitto)
- Audio stack (ALSA + PipeWire)
- OTA update daemon

**Filesystem:**
- Read-only rootfs (squashfs)
- Writable agent data partition (ext4)
- Separate OTA partition (A/B scheme)

**Boot flow:**
```
U-Boot → Kernel → Custom Init → EvoClaw Agent
```

**Total size:** ~30MB image, ~100MB with data partition.

### ClawOS Core (Phase 2)

A **custom microkernel OS** built on seL4 or a custom kernel.

**Why microkernel?**
- **Minimal trusted code** (kernel is <10K LOC)
- **Capability-based security** (no ambient authority)
- **Formal verification possible** (seL4 is mathematically proven correct)
- **Agent isolation** (each agent is a separate address space)

**Design:**
- **Microkernel:** seL4 or custom L4-family kernel
- **Agent runtime:** Runs as a system service (in userspace)
- **Drivers:** Minimal, moved to userspace
- **IPC:** Capability-based shared memory + message passing

**Boot flow:**
```
Bootloader → Microkernel → Agent Runtime Service → EvoClaw Agent
```

**Total size:** <5MB image, <50MB with runtime and data.

### ClawOS Full (Phase 3)

Full agent OS with **distributed agent mesh** and **ClawChain integration**.

**Features:**
- Multi-agent coordination primitives
- Distributed agent discovery (mDNS/gossip)
- ClawChain client (on-device blockchain node)
- Hardware attestation (TPM/TrustZone)
- Secure boot to root of trust

**Target:** 2027+, when agent ecosystems mature.

---

## Why "Be Water"?

Bruce Lee's famous philosophy: **"Be water, my friend."**

Water adapts to any container — poured into a cup, it becomes the cup. Poured into a bottle, it becomes the bottle.

EvoClaw agents **run anywhere**:
- Linux, macOS, Windows
- Raspberry Pi, x86, ARM
- Docker, Kubernetes, bare metal

**But water flows fastest in the right channel.**

ClawOS is that channel — purpose-built for agents, no friction, no bloat.

> **"Be water, my agent."** 🌊

---

## Status: Parked, But Real

ClawOS is **not vaporware**. It's a real design, grounded in proven technology (Buildroot, seL4, Yocto). But it's **parked** while we focus on EvoClaw.

**Why park it?**
1. **EvoClaw first:** Need to prove the agent evolution model works before building a dedicated OS
2. **Community validation:** Need real users/contributors to validate the need
3. **Resource focus:** Small team, big vision — one thing at a time

**When to resume?**
- EvoClaw has 50+ stars (proves interest)
- Companion use case validated (agents on Pi Zero work well)
- Contributors interested in embedded Linux/kernel work

ClawOS will happen. Just not today.

---

## Join Us

Have ideas? Want to help design the agent OS of the future?

- **Discussions:** [github.com/clawinfra/clawos/discussions](https://github.com/clawinfra/clawos/discussions)
- **Issues:** [github.com/clawinfra/clawos/issues](https://github.com/clawinfra/clawos/issues)
- **Active work:** Contribute to [EvoClaw](https://github.com/clawinfra/evoclaw) today

**Be water. Build the future.** 🌊
