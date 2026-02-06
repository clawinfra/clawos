# Contributing to ClawOS

Thank you for your interest in ClawOS! 🌊

ClawOS is currently **parked** while we focus on [EvoClaw](https://github.com/clawinfra/evoclaw), but we're actively collecting ideas, designs, and feedback for when development resumes.

---

## Current Status: Parked 🅿️

**What "parked" means:**
- No active development on ClawOS code right now
- We're focusing on EvoClaw (the agent framework that ClawOS will host)
- We're collecting ideas, designs, and community feedback
- When EvoClaw matures, ClawOS development will resume

**How you can help now:**
1. **Contribute to EvoClaw** — the agent framework needs to work before we build an OS for it
2. **Share ideas** — design feedback, hardware suggestions, use cases
3. **Experiment** — try running EvoClaw on embedded devices, report findings
4. **Design hardware** — create 3D-printable cases, circuit boards, BOMs

---

## Ways to Contribute

### 1. 💡 Share Ideas (GitHub Discussions)

We're collecting ideas for ClawOS design and implementation.

**Topics we're interested in:**
- **Use cases:** What would you build with ClawOS?
- **Hardware:** What devices should we support?
- **Features:** What OS-level features do agents need?
- **Security:** How should agent attestation work?
- **Networking:** How should agents discover and coordinate?

**How to contribute:**
- Go to [Discussions](https://github.com/clawinfra/clawos/discussions)
- Start a new discussion or join an existing one
- Be thoughtful and constructive
- Share your expertise (embedded Linux, kernel dev, hardware design, etc.)

---

### 2. 🐛 File Issues (Feature Requests)

Have a specific feature request or hardware request? File an issue!

**Use issue templates:**
- **Feature request:** New OS feature, tool, or improvement
- **Hardware request:** Request support for new device or board

**How to contribute:**
- Go to [Issues](https://github.com/clawinfra/clawos/issues)
- Click "New Issue" and select a template
- Fill out the template completely
- Be specific: "Support Raspberry Pi 5" is better than "Support more hardware"

---

### 3. 🛠️ Contribute to EvoClaw (Active Development)

ClawOS is parked, but **EvoClaw is active!**

EvoClaw is the agent framework that ClawOS will host. If you want to contribute code today:
- Go to [EvoClaw repo](https://github.com/clawinfra/evoclaw)
- Read the contributing guide
- Pick an issue or propose a feature
- Submit a pull request

**Why contribute to EvoClaw?**
- It's active development (not parked)
- Your code will run on ClawOS when it's ready
- You'll shape the agent framework that ClawOS is built for

---

### 4. 🔬 Experiment & Share Findings

Try running EvoClaw agents on embedded devices and share your results.

**Experiments we'd love to see:**
- **EvoClaw on Pi Zero 2W:** How does it perform? What's the RAM usage? Boot time?
- **EvoClaw on ESP32-S3:** Can you port it? What are the limitations?
- **Audio latency:** How fast is STT/TTS on different hardware?
- **Power usage:** How long does a Pi Zero run on battery?
- **Custom hardware:** Did you build a companion device? Share your design!

**How to share:**
- Write a blog post or GitHub gist
- Post in [Discussions](https://github.com/clawinfra/clawos/discussions)
- Tweet with #ClawOS and tag @clawinfra (when Twitter account exists)

---

### 5. 🎨 Design Hardware

Design custom hardware for ClawOS and share your designs.

**Ideas:**
- **Companion device case:** 3D-printable case for Pi Zero + mic + speaker
- **Wearable case:** Pendant or wrist-mounted case for ESP32-S3
- **Sensor node:** Weatherproof case for outdoor sensors
- **PCB:** Custom board with Pi Zero + audio codec + power management

**How to share:**
- Post STL files, PCB designs (KiCad/Eagle), or BOMs in [Discussions](https://github.com/clawinfra/clawos/discussions)
- Include photos, assembly instructions, and cost breakdown
- License your design (recommend: CC BY-SA 4.0 or Apache 2.0)

---

### 6. 📚 Write Documentation

Help us document ClawOS design, use cases, and tutorials.

**Documentation we need:**
- **Tutorials:** "How to run EvoClaw on Pi Zero 2W"
- **Comparisons:** "ClawOS Lite vs Raspbian vs Ubuntu Core"
- **Use cases:** "Building a voice assistant with ClawOS"
- **Hardware guides:** "Best microphones for companion agents"

**How to contribute:**
- Write a doc in Markdown
- Post in [Discussions](https://github.com/clawinfra/clawos/discussions) or submit a PR to `docs/`
- We'll review and merge (or link to your blog)

---

## Areas We Need Help

When ClawOS development resumes, we'll need contributors with expertise in:

### Embedded Linux
- **Buildroot/Yocto experience:** Building custom Linux distros
- **Kernel configuration:** Minimizing kernel size and boot time
- **Device tree:** Configuring hardware on ARM SBCs
- **Init systems:** Writing custom init (replacing systemd)

### Kernel Development
- **Microkernel design:** seL4, L4, or custom kernel design
- **Driver development:** Writing userspace drivers for audio, WiFi, GPIO
- **IPC mechanisms:** Shared memory, message passing, capability-based security
- **Real-time scheduling:** Low-latency scheduling for audio and sensors

### Audio & Multimedia
- **ALSA/PipeWire:** Audio stack configuration and optimization
- **STT/TTS:** Integrating Whisper, Piper, or other speech tools
- **Latency optimization:** Reducing mic-to-speaker latency
- **VAD:** Voice activity detection and wake word systems

### Hardware Design
- **PCB design:** Custom boards for agents (KiCad, Eagle, Altium)
- **3D modeling:** Cases and enclosures (Fusion 360, OpenSCAD, Blender)
- **Power management:** Battery charging, low-power modes
- **Audio hardware:** Microphone selection, speaker amps, I2S DACs

### Security
- **Secure boot:** UEFI, U-Boot, TPM integration
- **Cryptography:** Signing agent genomes, verifying updates
- **Capability-based security:** seL4-style access control
- **Hardware security:** TrustZone, SGX, secure enclaves

### Documentation & Community
- **Technical writing:** Explaining complex concepts clearly
- **Tutorials:** Step-by-step guides for users and developers
- **Community management:** Answering questions, triaging issues
- **Marketing:** Spreading the word about ClawOS

---

## Code of Conduct

ClawOS follows the **Contributor Covenant Code of Conduct**.

**TL;DR:**
- Be respectful and inclusive
- No harassment, discrimination, or trolling
- Focus on constructive feedback
- We're here to build cool stuff together

Full text: [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/)

---

## Development Process (When Active)

*(This section is for when ClawOS development resumes.)*

### 1. Fork & Clone
```bash
git clone https://github.com/YOUR_USERNAME/clawos.git
cd clawos
```

### 2. Create a Branch
```bash
git checkout -b feature/your-feature-name
```

### 3. Make Changes
- Write code
- Test thoroughly
- Follow existing code style
- Write commit messages that explain *why*, not just *what*

### 4. Submit a Pull Request
- Push your branch to GitHub
- Open a PR against `main` branch
- Fill out the PR template
- Respond to review feedback

### 5. Review & Merge
- Maintainers will review your PR
- CI will run tests (when we have CI)
- Once approved, we'll merge!

---

## Licensing

**Code:** Apache 2.0 (permissive, allows commercial use)

**Documentation:** CC BY-SA 4.0 (share-alike)

**Hardware designs:** Recommended: CC BY-SA 4.0 or Apache 2.0

By contributing, you agree to license your contributions under the same terms.

---

## Communication Channels

**GitHub:**
- [Discussions](https://github.com/clawinfra/clawos/discussions) — ideas, questions, feedback
- [Issues](https://github.com/clawinfra/clawos/issues) — feature requests, bugs (when active)
- [Pull Requests](https://github.com/clawinfra/clawos/pulls) — code contributions (when active)

**Future channels (when community grows):**
- Discord server (for real-time chat)
- Mailing list (for announcements)
- Twitter/X (@clawinfra — TBD)

---

## Recognition

Contributors will be recognized in:
- **CONTRIBUTORS.md** (list of all contributors)
- **Release notes** (for each release)
- **README badges** (top contributors)

We appreciate every contribution, big or small! 🙏

---

## Questions?

**General questions:** Open a [Discussion](https://github.com/clawinfra/clawos/discussions)

**Specific issues:** File an [Issue](https://github.com/clawinfra/clawos/issues)

**Want to chat?** *(Future: Discord link here when available)*

---

**Thank you for helping build the OS for evolving agents!** 🌊

**Be water. Contribute.** 🌊
