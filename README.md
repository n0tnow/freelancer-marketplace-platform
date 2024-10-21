FeeLess
![generated-image](https://github.com/user-attachments/assets/46d584ea-7c18-4ddc-965c-6fbb0963c048)

Hello. I am Bilal Kaya, a third-year Software Engineering student at Beykoz University, studying on a full scholarship. With skills in C++, C#, Python, OOP, and web development frameworks like Django, I have built several projects, including a Bakery-Cafe Management System and a Fitness Center Management System. I am also proficient in developing RESTful APIs and working with SQL databases. Recently, I participated in the Rise In Stack Bootcamp, where I deepened my knowledge of modern technologies and blockchain development. My goal is to build innovative, impactful solutions in the tech industry.




Vision

The vision of FeeLess is to create a transparent, decentralized marketplace where freelancers and employers can collaborate without the burden of high fees or intermediaries. FeeLess empowers individuals to focus on the work that matters, reducing financial barriers and enabling secure, automated transactions in a trusted environment. This project aims to reshape the freelancing ecosystem by offering a frictionless platform that leverages blockchain technology to build trust and transparency, fostering better professional relationships.

Project Description

FeeLess is a blockchain-powered freelance marketplace designed to facilitate transparent, secure, and fee-less interactions between freelancers and employers.

Built on the Stellar blockchain, FeeLess leverages smart contracts through the Soroban SDK to ensure that every agreement is automatically enforced, payments are securely held in escrow, and disputes are minimized. The platform focuses on creating a seamless user experience where freelancers can submit proposals, and employers can safely manage their projects. Payment flows are automated and secure, thanks to Stellar’s efficient network, enabling professionals to focus on their work without worrying about high platform fees or financial risks.


Future Plans
FeeLess is just the beginning of what can be achieved by leveraging blockchain technology for freelance marketplaces. Here are some of the future plans for the platform:

Decentralized Dispute Resolution:

Implement a peer-to-peer arbitration system where neutral third parties can help resolve disputes between freelancers and employers, ensuring fair outcomes without traditional intermediaries.
Multi-Currency Support:

Expand the platform to allow transactions in multiple cryptocurrencies, providing greater flexibility for users worldwide.
Reputation and Rating System:

Build an on-chain reputation system that tracks user performance and reliability. Ratings would be immutable and verifiable through blockchain, ensuring a transparent history for both freelancers and employers.
Mobile and Web App Integration:

Develop mobile and web applications to enhance user experience, allowing seamless interaction with the FeeLess platform, notifications for job updates, and instant messaging between freelancers and employers.
Global Freelance Ecosystem:

Expand FeeLess into a global ecosystem by forming partnerships with local freelance communities and integrating localized services, making the platform accessible and useful to professionals around the world.
Full Decentralization:

Transition towards a fully decentralized governance model where users can vote on platform upgrades, fee structures, and new features using a native token.

The Tech We Use
Rust & Web3

Installation
Clone the repository:

git clone https://github.com/your-username/feeless.git
cd feeless
Build the Smart Contract:

cargo build --target wasm32-unknown-unknown --release
Deploy to Stellar Testnet: Ensure you have your testnet secret key ready, then deploy the contract:

soroban contract deploy --wasm target/wasm32-unknown-unknown/release/freelancer_marketplace_lib.wasm \
--network testnet \
--source-account YOUR_SECRET_KEY \
--network-passphrase "Test SDF Future Network ; October 2022" \
--rpc-url https://rpc-futurenet.stellar.org:443
Interact with the Contract: You can now interact with the deployed contract by calling its functions (register users, create jobs, etc.) using the Soroban CLI.


