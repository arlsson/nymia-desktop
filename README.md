# Nymia - Decentralized Chat Application

![Nymia Marketing Header](https://www.imgdumper.nl/uploads9/6856d3cbbf18b/6856d3cbbabe0-marketing-header.png)

Nymia is a secure, decentralized desktop chat application that enables peer-to-peer messaging using the Verus blockchain (including all other PBaaS-chains such as CHIPS, vDEX, vARRR). Unlike traditional chat applications that rely on central servers, Nymia uses blockchain technology for identity and message transport, giving users complete control over their data and communications.

## Key Features

- **Decentralized Messaging**: Messages are sent directly through the Verus blockchain, eliminating the need for central servers
- **Self-Sovereign Identity**: Uses VerusID for blockchain-based identity management
- **End-to-End Security**: Messages are cryptographically signed and verified to prevent spoofing
- **Privacy-First**: Optional local storage with user-controlled data persistence
- **Serverless Architecture**: No corporate servers or data collection

## How It Works

1. **Connect** to your Verus daemon (wallet) using RPC credentials
2. **Login** with your VerusID (blockchain-based identity)
3. **Chat** by sending encrypted messages as blockchain transactions
4. **Verify** message authenticity through cryptographic signatures

## Prerequisites

- [Node.js](https://nodejs.org/) (v18 or higher)
- [Yarn](https://yarnpkg.com/) package manager
- [Rust](https://rustup.rs/) toolchain
- Running Verus daemon/wallet with RPC access

## Installation

```bash
# Clone the repository
git clone https://github.com/Meyse/nymia-desktop.git
cd nymia-desktop

# Install dependencies
yarn install
```

## Development

```bash
# Start development server
yarn tauri dev
```

## Building

```bash
# Build for production
yarn tauri build
```

## Tech Stack

- **Frontend**: SvelteKit + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri
- **Blockchain**: Verus Protocol
- **Identity**: VerusID

## Important Disclaimers

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

**Security Notice**: Nymia is experimental software that handles blockchain transactions. Users are responsible for:
- Securing their own private keys
- Understanding the risks of blockchain transactions
- Verifying all transactions and communications
- Maintaining backups of important data

**No Financial Advice**: This software is a communication tool only and does not provide financial, investment, or legal advice.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
