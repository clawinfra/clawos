# ClawOS Hardware Targets

ClawOS is designed for **resource-constrained edge devices**, not servers or desktops.

This document covers target hardware platforms, requirements, and reference designs.

---

## Hardware Tiers

ClawOS supports three tiers of hardware:

### Tier 1: Primary Targets
Official support, tested extensively, guaranteed to work.

- **Raspberry Pi Zero 2W** (ClawOS Lite)
- **ESP32-S3** (ClawOS Core, future)

### Tier 2: Supported Platforms
Tested, supported, but not primary focus.

- Raspberry Pi 3 Model B/B+
- Raspberry Pi 4 Model B
- Raspberry Pi 5
- BeagleBone Black
- BeagleBone AI
- NVIDIA Jetson Nano

### Tier 3: Community Targets
Community-maintained, may not be fully tested.

- x86 embedded boards (Intel NUC, Fitlet, etc.)
- RISC-V boards (VisionFive, Lichee, etc.)
- Custom hardware (bring your own board)

---

## Tier 1: Primary Targets

### Raspberry Pi Zero 2W

**Why it's perfect for ClawOS:**
- **Cheap:** $15 USD
- **Small:** 65mm × 30mm
- **Low power:** ~300mA @ 5V (~1.5W)
- **Quad-core:** BCM2710 (Cortex-A53 @ 1 GHz)
- **RAM:** 512 MB
- **Connectivity:** WiFi 802.11n, Bluetooth 4.2
- **GPIO:** 40-pin header (compatible with full-size Pi)
- **Camera:** CSI connector (Pi Camera support)
- **Audio:** I2S support (via GPIO)

**ClawOS Lite specifications:**
- **OS image:** ~30 MB (compressed SquashFS)
- **RAM usage:** ~150 MB (OS + agent)
- **Boot time:** <2 seconds (power-on to agent-ready)
- **Storage:** 8 GB microSD minimum (16 GB recommended)

**Use cases:**
- **Companion agents:** Always-on voice assistant
- **Home automation:** Sensor hub, smart speaker
- **Edge AI:** Local inference (lightweight models)

**Bill of Materials (BOM) for companion device:**

| Component | Description | Price (USD) |
|-----------|-------------|-------------|
| Raspberry Pi Zero 2W | Main board | $15 |
| microSD card (16 GB) | Storage | $5 |
| USB microphone | Audio input | $8 |
| Mini speaker (3.5mm) | Audio output | $5 |
| USB-C power supply (5V 2.5A) | Power | $6 |
| Case (3D printed or acrylic) | Enclosure | $3 |
| **Total** | | **$42** |

**Optional:**
- Pi Camera Module 3 ($25) — for visual agents
- I2S microphone (SPH0645) ($5) — better audio quality
- 18650 battery + charging board ($10) — portable power

---

### ESP32-S3 (Future Target for ClawOS Core)

**Why it's the future:**
- **Tiny:** $5-10 USD, fits in wearables
- **Ultra-low power:** <100mA active, <10μA deep sleep
- **Dual-core:** Xtensa LX7 @ 240 MHz
- **RAM:** 512 KB SRAM (+ 2-8 MB PSRAM optional)
- **Flash:** 4-16 MB
- **Connectivity:** WiFi 802.11n, Bluetooth 5.0 (LE)
- **AI acceleration:** Vector instructions for ML inference
- **Audio:** I2S, PDM microphone support

**ClawOS Core specifications:**
- **OS image:** <5 MB (microkernel + agent runtime)
- **RAM usage:** ~200 KB (kernel) + ~300 KB (agent)
- **Boot time:** <500ms (power-on to agent-ready)
- **Storage:** 8 MB flash minimum

**Use cases:**
- **Wearables:** Smart watch, earbuds, pendant
- **Sensor agents:** Battery-powered environmental monitors
- **Swarm agents:** Coordinated multi-device systems

**Bill of Materials (BOM) for wearable agent:**

| Component | Description | Price (USD) |
|-----------|-------------|-------------|
| ESP32-S3-DevKitC | Dev board (8 MB flash, 2 MB PSRAM) | $10 |
| MEMS microphone (INMP441) | I2S digital mic | $2 |
| Speaker (8Ω 0.5W) | Audio output | $2 |
| LiPo battery (500mAh) | Power | $5 |
| Battery charger (TP4056) | USB charging | $1 |
| 3D-printed case | Enclosure | $2 |
| **Total** | | **$22** |

**Status:** ClawOS Core for ESP32-S3 is **future work** (Phase 2+). Requires microkernel port to Xtensa architecture.

---

## Tier 2: Supported Platforms

### Raspberry Pi 3/4/5

**Advantages over Pi Zero 2W:**
- **More power:** Quad-core Cortex-A53/A72/A76 @ 1.5-2.4 GHz
- **More RAM:** 1-8 GB
- **Better I/O:** Gigabit Ethernet, USB 3.0, dual HDMI (Pi 4/5)
- **Better WiFi:** 802.11ac (Pi 3B+/4/5)

**When to use:**
- Need more compute (larger AI models, multiple agents)
- Need wired Ethernet (reliability)
- Need USB peripherals (webcam, external storage)

**Downsides:**
- **More expensive:** $35-80 USD
- **More power:** 2-5W (vs 1.5W for Pi Zero 2W)
- **Bigger:** Full-size board

**ClawOS Lite works identically** — same 30 MB image, same boot time.

---

### BeagleBone Black / BeagleBone AI

**Why BeagleBone:**
- **Industrial-grade:** More robust than Pi (wider temp range, better EMI)
- **PRU co-processors:** Real-time microcontrollers on-chip (useful for sensors)
- **eMMC storage:** Onboard flash (no SD card needed)

**BeagleBone Black:**
- **CPU:** AM3358 (Cortex-A8 @ 1 GHz)
- **RAM:** 512 MB
- **Storage:** 4 GB eMMC + microSD slot
- **Price:** $60 USD

**BeagleBone AI:**
- **CPU:** AM5729 (dual Cortex-A15 @ 1.5 GHz)
- **GPU:** SGX544 (PowerVR)
- **AI accelerator:** C66x DSPs + EVEs (5 TFLOPS)
- **RAM:** 1 GB
- **Price:** $120 USD

**Use cases:**
- **Industrial edge:** Factory sensors, robotics
- **Vision agents:** On-device object detection (BeagleBone AI)

---

### NVIDIA Jetson Nano

**Why Jetson:**
- **GPU acceleration:** 128-core Maxwell GPU
- **AI inference:** TensorFlow, PyTorch, ONNX support
- **Camera support:** MIPI CSI (up to 4K resolution)

**Specs:**
- **CPU:** Quad-core Cortex-A57 @ 1.43 GHz
- **GPU:** 128-core Maxwell @ 921 MHz
- **RAM:** 2 GB or 4 GB
- **Price:** $99 USD (2GB), $129 USD (4GB, discontinued but available used)

**Use cases:**
- **Vision agents:** Real-time object detection, facial recognition
- **Multi-modal agents:** Vision + audio + language models

**ClawOS Lite support:** Experimental. Jetson uses custom NVIDIA drivers (not fully open source).

---

## Tier 3: Community Targets

### x86 Embedded Boards

**Examples:**
- Intel NUC (mini PC, $150-400)
- Fitlet2 (fanless industrial PC, $200-500)
- Up Board (x86 SBC with GPIO, $100-200)

**Why use x86:**
- **Compatibility:** Runs standard Linux software
- **Performance:** 2-4x faster than ARM SBCs
- **Storage:** M.2 SSD support (fast, reliable)

**Downsides:**
- **Power:** 10-20W (vs 1.5W for Pi Zero)
- **Cost:** 3-10x more expensive
- **Size:** Bigger than ARM SBCs

**ClawOS Lite support:** Yes, but not optimized for x86 power usage.

---

### RISC-V Boards

**Examples:**
- VisionFive 2 (quad-core RISC-V, $80)
- Lichee RV Dock (RISC-V with WiFi, $20)
- PineTab-V (RISC-V tablet, $200)

**Why RISC-V:**
- **Open architecture:** No licensing fees (unlike ARM)
- **Future-proof:** Growing ecosystem
- **Customizable:** Can design custom RISC-V chips

**Status:** Experimental. RISC-V ecosystem is young, drivers are immature.

**ClawOS Core future:** RISC-V is a natural fit for custom microkernels. Long-term target.

---

## Hardware Requirements

### Minimum Requirements (ClawOS Lite)

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 1 GHz ARM Cortex-A7+ or x86 | 1.5 GHz quad-core ARM Cortex-A53+ |
| RAM | 256 MB | 512 MB |
| Storage | 4 GB | 8 GB |
| Network | WiFi 802.11n or Ethernet | WiFi 802.11ac or Gigabit Ethernet |
| Audio | I2S or USB microphone + speaker | I2S MEMS mic + I2S DAC |

### Minimum Requirements (ClawOS Core, future)

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 100 MHz ARM Cortex-M4+ or Xtensa LX6+ | 240 MHz dual-core Xtensa LX7 |
| RAM | 128 KB SRAM | 512 KB SRAM + 2 MB PSRAM |
| Storage | 2 MB flash | 8 MB flash |
| Network | WiFi 802.11n or Bluetooth LE | WiFi 802.11n + Bluetooth 5.0 |
| Audio | PDM or I2S microphone | I2S MEMS mic + I2S DAC |

---

## Reference Designs

### Companion Device (Voice Assistant)

**Target hardware:** Raspberry Pi Zero 2W

**Components:**
- **Board:** Raspberry Pi Zero 2W ($15)
- **Microphone:** USB mini mic (e.g., Kinobo USB, $8)
- **Speaker:** 3.5mm mini speaker ($5) or I2S DAC + 4Ω speaker ($10)
- **Power:** USB-C 5V 2.5A ($6)
- **Case:** 3D-printed or laser-cut acrylic ($3-5)

**Optional:**
- **Camera:** Pi Camera Module 3 ($25) — for visual agents
- **Battery:** 18650 Li-ion + charging board ($10) — for portable use
- **Display:** 1.3" OLED I2C display ($8) — for status/faces

**Total cost:** $42 (basic) to $90 (fully loaded).

**Software stack:**
- ClawOS Lite (~30 MB)
- EvoClaw agent (Node.js or native)
- STT: Whisper.cpp or cloud API
- TTS: Piper or cloud API
- MQTT broker: Mosquitto

**3D printable case:** *(Community to design)*

---

### Wearable Agent (Smart Pendant)

**Target hardware:** ESP32-S3

**Components:**
- **Board:** ESP32-S3-DevKitC ($10) or custom PCB ($20)
- **Microphone:** INMP441 I2S MEMS mic ($2)
- **Speaker:** 8Ω 0.5W mini speaker ($2)
- **Battery:** 500mAh LiPo ($5)
- **Charger:** TP4056 USB charging module ($1)
- **Case:** 3D-printed pendant ($2)

**Optional:**
- **Vibration motor:** For haptic feedback ($1)
- **LED:** Neopixel RGB for status ($1)
- **Button:** Tactile switch for wake word override ($0.50)

**Total cost:** $22 (basic) to $30 (fully featured).

**Software stack:**
- ClawOS Core (<5 MB)
- EvoClaw agent (native or WASM)
- STT: Edge impulse or cloud API
- TTS: Edge TTS or cloud API

**3D printable case:** *(Community to design)*

---

### Multi-Agent Swarm (Sensor Network)

**Target hardware:** Multiple ESP32-S3 boards

**Use case:** Distributed environmental monitoring (temperature, humidity, CO2, motion).

**Per-node BOM:**
- **ESP32-S3:** $10
- **Sensors:** BME680 (temp/humidity/pressure/VOC, $15)
- **Power:** 2× AA batteries + holder ($2)
- **Case:** Weatherproof 3D-printed ($3)

**Total per node:** $30

**Network topology:**
- Each node runs ClawOS Core + sensor agent
- Agents communicate via MQTT over WiFi mesh
- Coordinator node (Pi Zero 2W) aggregates data

**Software stack:**
- ClawOS Core on ESP32-S3
- ClawOS Lite on coordinator Pi
- MQTT broker on coordinator
- Agents publish sensor data, coordinator analyzes

---

## Peripheral Recommendations

### Audio Input (Microphones)

**USB microphones (Pi Zero 2W, Pi 3/4/5):**
- **Kinobo USB Mini Mic:** $8, plug-and-play, decent quality
- **Blue Snowball iCE:** $50, studio quality (overkill for agents)

**I2S microphones (Pi, ESP32):**
- **SPH0645 (Adafruit):** $7, I2S MEMS mic, 24-bit, excellent quality
- **INMP441:** $2, I2S MEMS mic, 24-bit, good quality

**Recommendation:** I2S microphones (better quality, lower CPU usage, no USB overhead).

---

### Audio Output (Speakers)

**USB speakers (Pi):**
- Any USB speaker works, but adds latency and USB overhead

**3.5mm speakers (Pi):**
- **AmazonBasics Powered Speakers:** $15, plug into Pi's 3.5mm jack
- **Mini speakers:** $5, battery-powered, portable

**I2S DACs (Pi, ESP32):**
- **Adafruit I2S DAC:** $10, connects via GPIO, better audio quality
- **MAX98357A breakout:** $5, I2S DAC + 3W amp, drives 4-8Ω speakers directly

**Recommendation:** I2S DAC + passive speaker (best quality, no USB, low latency).

---

### Cameras

**Raspberry Pi:**
- **Pi Camera Module 3:** $25, 12 MP, autofocus, excellent quality
- **Pi HQ Camera:** $50, 12 MP, C/CS mount (interchangeable lenses)

**ESP32-S3:**
- **OV2640 camera module:** $5, 2 MP, 1600×1200, adequate for basic vision

**USB webcams (Pi):**
- Most USB webcams work, but add latency and USB overhead

**Recommendation:** Pi Camera Module 3 for vision agents (official support, low latency).

---

## Power Considerations

### Raspberry Pi Zero 2W

**Typical power consumption:**
- **Idle:** ~100-150 mA @ 5V (~0.5-0.75W)
- **Active (CPU 50%):** ~200-300 mA @ 5V (~1-1.5W)
- **Full load (CPU 100%):** ~400-500 mA @ 5V (~2-2.5W)

**Battery life (with 10,000 mAh power bank):**
- Idle: ~50-60 hours
- Active: ~25-30 hours
- Full load: ~15-20 hours

**Recommendation:** Use official Raspberry Pi USB-C power supply (5V 2.5A, $6).

---

### ESP32-S3

**Typical power consumption:**
- **Deep sleep:** ~10 μA (~0.00005W)
- **Light sleep (WiFi off):** ~2 mA (~0.01W)
- **Active (WiFi on):** ~80-100 mA (~0.4-0.5W)
- **Full load (CPU + WiFi):** ~150-200 mA (~0.75-1W)

**Battery life (with 500 mAh LiPo):**
- Deep sleep: ~5 years (impractical, other factors limit life)
- Light sleep: ~10 days
- Active: ~5-6 hours
- Full load: ~2.5-3 hours

**Use case:** Wake on voice (low-power voice activity detection), deep sleep between interactions.

---

## Custom Hardware

Want to design your own ClawOS-compatible board? Great!

**Minimum specs:**
- **CPU:** ARM Cortex-A7+ (for ClawOS Lite) or Cortex-M4+/Xtensa LX6+ (for ClawOS Core)
- **RAM:** 256 MB (Lite) or 128 KB (Core)
- **Storage:** 4 GB (Lite) or 2 MB (Core)
- **Network:** WiFi 802.11n or Bluetooth LE
- **Audio:** I2S or PDM microphone + DAC/amp

**Reference designs:**
- Raspberry Pi Zero 2W schematics (for ARM Cortex-A53 design)
- ESP32-S3 reference design (for Xtensa dual-core design)

**Community support:**
- Post your design in [Discussions](https://github.com/clawinfra/clawos/discussions)
- File a [hardware request](https://github.com/clawinfra/clawos/issues) for official support

---

## Summary

### Best for ClawOS Lite:
- **Raspberry Pi Zero 2W** — $15, 512 MB RAM, perfect for companion agents

### Best for ClawOS Core (future):
- **ESP32-S3** — $10, ultra-low power, perfect for wearables

### Best for vision agents:
- **Jetson Nano** — GPU acceleration for real-time object detection

### Best for industrial:
- **BeagleBone Black/AI** — ruggedized, eMMC storage, PRU co-processors

---

**Questions about hardware?** Open a [Discussion](https://github.com/clawinfra/clawos/discussions) or [hardware request issue](https://github.com/clawinfra/clawos/issues).

**Be water. Pick the right container.** 🌊
