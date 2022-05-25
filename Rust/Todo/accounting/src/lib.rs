/*
currency: USD -- 货币单位

- me :: String
    - all -- 资产 :: isize
    - how many -- 权益 : u32
    - in -- 入 : u32 * u32
        - how much -- 价值 :: u32
        - how many -- 数量 :: u32
    - out -- 负债 :: u32



- when -- 日期 :: String
    - when -- 结束日期 :: String
    - what -- 商品名称 :: String
    - what -- 交易方式 :: String
        - how much -- 价值 :: u32
            - how much -- 折扣 :: u8 :: 1~100
            - how many -- 数量 :: u32 :: 1~N
    - who :: [String,String]
        - who(s) :: 我(团体/个人) :: String
        - who(s) :: 别人(团体/个人) :: String
        - tags :: Option<Vec<String>>
            - 商品种类
            - 商品满意度
            - 何地
    - what -- 何种渠道 :: String
            ...
        - note -- 其他 :: String

2000-01-01 ! dog : $1 : {"一条狗 两条狗", "没有头"}
# date
    = 2000-01-01 -- 2000-01-02
    # dog
      = $10
      = 7
    # 人
      +买方
        :我自己
            ::现金 -$10
            ::卡 -$50
            ::捡到的钱 -$10
      +卖方
        :他自己
            ::现金 +$20
        : 他爸
            ::卡 +$50
    # tags
      - [dog, 狗, 没头]




* = 已核实
! = 未核实




资产(Assets) : isize
        现金
        银行存款
        有价证券
        ...
    收入(Income) :   ------------|
        工资                     + = isize
        奖金等；                 |
    支出(Expenses) : ------------|
        就餐
        购物
        旅行
        ...
    负债(Liabilities) : usize
        信用卡
        贷款
            房贷
            车贷
        ...
    权益(Equity) : usize
        可以当成净资产

资产=负债+所有者权益
所有者权益 = 资产-负债
(收入 + 负债) + (资产 + 费用) + 权益 = 0



https://gitpress.io/c/beancount-tutorial/beancount-tutorial-1





edit = vim / cli

*/

/*
accounts :: account1 :: account2
            |           |
           / \         / \





*/

// https://github.com/ebcrowder/rust_ledger/blob/main/src/ledger.rs

pub mod utils {
    pub mod time;
}
