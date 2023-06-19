use ethers::{
    contract::{abigen, Abigen, ContractFactory},
    prelude::*,
    utils,
};

abigen!(
        IERC20,
        r#"[
            function totalSupply() external view returns (uint256)
            function balanceOf(address account) external view returns (uint256)
            function transfer(address recipient, uint256 amount) external returns (bool)
            function allowance(address owner, address spender) external view returns (uint256)
            function approve(address spender, uint256 amount) external returns (bool)
            function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
            event Transfer(address indexed from, address indexed to, uint256 value)
            event Approval(address indexed owner, address indexed spender, uint256 value)
        ]"#,
    );


abigen!(
        IUniswapV2Router01,
        r#"[
            function factory() external pure returns (address)
            function WETH() external pure returns (address)
        ]"#,
    );

abigen!(
        IUniswapV2Factory,
        r#"[
            function feeTo() external view returns (address)
            function feeToSetter() external view returns (address)
            function getPair(address tokenA, address tokenB) external view returns (address)
            function allPairs(uint) external view returns (address)
            function allPairsLength() external view returns (uint)
        ]"#,
    );

abigen!(
        IUniswapV2Pair,
        r#"[
            function name() external pure returns (string memory)
            function symbol() external pure returns (string memory)
            function token0() external view returns (address)
            function token1() external view returns (address)
            function decimals() external pure returns (uint8)
            function totalSupply() external view returns (uint)
            function balanceOf(address owner) external view returns (uint)
            function allowance(address owner, address spender) external view returns (uint)
            function approve(address spender, uint value) external returns (bool)
            function transfer(address to, uint value) external returns (bool)
            function transferFrom(address from, address to, uint value) external returns (bool)
        ]"#,
    );