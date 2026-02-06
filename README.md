# ClawOS 🌊

> **"The OS that flows like water"**

A minimal, agent-native operating system purpose-built for [EvoClaw](https://github.com/clawinfra/evoclaw) agents.

**Adapts. Evolves. Everywhere.**

---

## 🔮 Vision Phase

ClawOS is currently **parked** in the vision phase. We're focusing on maturing the [EvoClaw](https://github.com/clawinfra/evoclaw) agent framework first. When EvoClaw proves its evolution model and gains traction, ClawOS will resume development as the optimal runtime environment.

---

## What is ClawOS?

ClawOS is an operating system designed from the ground up with **agents as first-class citizens**, not just processes. While EvoClaw runs on any OS (Linux, macOS, Windows), ClawOS removes the friction between agent and hardware — creating the most optimized container for evolving AI agents.

### The Water Philosophy

> **"Be water, my agent"** — Bruce Lee (adapted)

Water adapts to any container. EvoClaw agents run anywhere. But water flows fastest in a channel designed for it. That's ClawOS.

- **Adapts:** Runs on everything from ESP32 to x86
- **Evolves:** Built-in genome update system, agents evolve over-the-air
- **Everywhere:** Minimal footprint, boots in <500ms, fits in 5-30MB

---

## The Stack

ClawOS is the foundation for the Claw ecosystem:

```
┌─────────────────────────────────────┐
│   Persona / Genome (evolving AI)    │
├─────────────────────────────────────┤
│   EvoClaw Agent Framework           │
├─────────────────────────────────────┤
│   ClawOS (this repo)                │
├─────────────────────────────────────┤
│   Hardware (Pi, ESP32, x86, ARM)    │
└─────────────────────────────────────┘
         ↕ coordinates via
    ClawChain (agent blockchain)
```

- **[EvoClaw](https://github.com/clawinfra/evoclaw):** The agent framework with evolving genomes
- **ClawOS:** The OS optimized for running EvoClaw agents
- **ClawChain:** The blockchain for agent coordination and governance (future)

---

## Three Phases

### Phase 1: ClawOS Lite
**Target:** When EvoClaw has 50+ stars

A minimal Linux distribution built with Buildroot/Yocto:
- Custom agent-first init system (not systemd)
- Pre-installed EvoClaw runtime, MQTT broker, audio stack
- Read-only root filesystem + writable agent data partition
- OTA updates for agents AND genomes
- **Total image:** ~30MB
- **Boot time:** <2 seconds
- **Targets:** Raspberry Pi Zero 2W, Pi 3/4/5

### Phase 2: ClawOS Core
**Target:** When EvoClaw has 100+ contributors

A custom microkernel OS:
- Built on seL4 or custom microkernel
- Agent runtime as an OS service
- Zero-copy IPC between agents
- Hardware capability-based security
- Real-time scheduling for sensor/actuator agents
- **Total image:** <5MB
- **Boot time:** <500ms
- **Targets:** ESP32-S3, ARM Cortex-M/A, custom silicon

### Phase 3: ClawOS Full
**Target:** 2027+

Full agent OS with:
- Secure boot + agent attestation
- Distributed agent mesh networking
- Hardware abstraction layer for sensors/actuators
- Multi-agent isolation and coordination primitives
- Integration with ClawChain for agent governance

---

## Why ClawOS?

### Problems with general-purpose OSes:
- **Bloat:** Gigabytes of unused libraries and services
- **Boot time:** 10+ seconds to start an agent
- **Security:** No agent-specific isolation or attestation
- **No agent primitives:** Agents are just processes with no OS-level support
- **Resource waste:** 500MB+ RAM just for the OS

### ClawOS design principles:
1. **Agent as first-class citizen** — not just a process
2. **Boot in <500ms** — agents start instantly
3. **Minimal image** — <30MB (Lite) or <5MB (Core)
4. **Secure boot + agent attestation** — verify agent genomes
5. **Zero-copy IPC** — agents communicate efficiently
6. **Evolution runtime** — built-in genome update system
7. **OTA genome updates** — agents evolve over-the-air
8. **Hardware abstraction** — sensors/actuators as first-class I/O

---

## Target Hardware

**Tier 1 (primary targets):**
- Raspberry Pi Zero 2W (ClawOS Lite)
- ESP32-S3 (ClawOS Core, future)

**Tier 2 (supported):**
- Raspberry Pi 3/4/5
- BeagleBone Black/AI
- NVIDIA Jetson Nano

**Tier 3 (community-driven):**
- x86 embedded boards
- RISC-V boards
- Custom silicon

See [docs/HARDWARE.md](docs/HARDWARE.md) for detailed specs and reference designs.

---

## Documentation

- [📖 Vision](docs/VISION.md) — Why we're building ClawOS
- [🏗️ Architecture](docs/ARCHITECTURE.md) — Technical design and implementation
- [🔧 Hardware](docs/HARDWARE.md) — Target platforms and reference designs
- [🗺️ Roadmap](docs/ROADMAP.md) — Development phases and timeline
- [🤝 Contributing](CONTRIBUTING.md) — How to get involved

---

## Status: PARKED 🅿️

ClawOS is not actively developed yet. We're focusing on:
1. **EvoClaw framework** — proving the evolution model works
2. **Community building** — gathering contributors and use cases
3. **Hardware validation** — testing EvoClaw on target platforms

**When to resume:** When EvoClaw demonstrates stable agent evolution and the community validates the need for a dedicated OS.

---

## Contributing

We're collecting ideas, design feedback, and hardware expertise!

- **💡 Ideas:** Open a [Discussion](https://github.com/clawinfra/clawos/discussions)
- **🐛 Issues:** File feature requests or hardware requests via [Issues](https://github.com/clawinfra/clawos/issues)
- **🛠️ Active work:** Contribute to [EvoClaw](https://github.com/clawinfra/evoclaw) today

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

## Philosophy

ClawOS isn't about control — it's about **removing friction** between agent and hardware.

EvoClaw runs on ANY OS. ClawOS is just the most optimized container.

Open source. Community-driven. Agent-native.

**Be water, my agent.** 🌊

---

## License

Apache 2.0 — See [LICENSE](LICENSE)

---

**Built by the Claw community** • [EvoClaw](https://github.com/clawinfra/evoclaw) • [ClawChain](https://github.com/clawinfra) (coming soon)
