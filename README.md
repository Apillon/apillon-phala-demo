# Schrödinger’s-nft-template-vuejs

Template will help you set up website, where users can decrypt and download files, that were assigned to their NFTs.

User flow:

- User connects it's wallet
- Website lists NFTs in that wallet
- User selects NFT and uses it to decrypt and download file from IPFS

## Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur) + [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin).

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin) to make the TypeScript language service aware of `.vue` types.

If the standalone TypeScript plugin doesn't feel fast enough to you, Volar has also implemented a [Take Over Mode](https://github.com/johnsoncodehk/volar/discussions/471#discussioncomment-1361669) that is more performant. You can enable it by the following steps:

1. Disable the built-in TypeScript Extension
   1. Run `Extensions: Show Built-in Extensions` from VSCode's command palette
   2. Find `TypeScript and JavaScript Language Features`, right click and select `Disable (Workspace)`
2. Reload the VSCode window by running `Developer: Reload Window` from the command palette.

## Customize configuration

See [Vite Configuration Reference](https://vitejs.dev/config/).

## Project Setup

```sh
npm install
```

### Compile, Preview and Hot-Reload for Development

```sh
npm run dev
```

### Type-Check, Compile and Minify for Production

```sh
npm run build
```

### Lint with [ESLint](https://eslint.org/)

```sh
npm run lint
```

## Configure

Before the template can interact with the Phala Schrödinger’s NFT Smart Contract, it needs to be manually configured.

To configure the template, do the following:

1. Create .env file
2. Set up configuration as displayed in [env sample](./.env.sample)
3. VITE_NFT_ADDRESS is address of NFT collection. User needs to have an NFT, and file is linked directly to that NFT.
4. VITE_CONTRACT_ADDRESS is address of Phala smart contract, visible in Apillon console.

Example:

```sh
# NFT Collection address
VITE_NFT_ADDRESS=0xC280459C8C8bd3d1870A69e9b547230C5aFF4022

# NFT Collection CHAIN ID
# Moonbase
VITE_NFT_CHAIN_ID=0x507
# Moonbeam
#VITE_NFT_CHAIN_ID=0x504
# Astar
#VITE_NFT_CHAIN_ID=0x250

# PHALA CONTRACT ADDRESS
VITE_CONTRACT_ADDRESS=0xd61a5c16cec10ca3ace980388765e5a5fc746e3cddaae3242330ce054c895cb9
```

Once you have updated the .env file, save it. Now the website files are ready to be deployed.

## Deploy to Apillon Hosting

To deploy the website on Apillon hosting you need to build project with a command below:

```sh
npm run build
```

And then deploy folder **dist** according to this documentation: [Wiki](https://wiki.apillon.io/build/3-hosting-api.html)

### Basic

1. If not already, register to [Apillon.io](https://app.apillon.io)
2. Log in to Apillon console and create new website inside your project.
3. Select all files of your website (as configured in the previous step) and use drag&drop action to pull the files into the Hosting bucket
4. Once the files are uploaded, push them to Staging and finally to the Production
5. Add your custom domain (as displayed in the dashboards UI)
6. Review your newly deployed website
