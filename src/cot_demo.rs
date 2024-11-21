use std::{fs::File, io::Write};

use rig::completion::{Chat, Message};

use crate::untils::create_client_from_env;


#[tokio::test]
async fn test_CoT() -> Result<(), anyhow::Error> {
    // 加载环境变量
    dotenv::dotenv().ok();
    
    // 根据环境变量创建 OpenAI 客户端
    let client = create_client_from_env();
    
    // 创建 CoT 代理
    let mut cot_agent = client.agent("gpt-4")
        .preamble("
            You are an intelligent assistant capable of performing complex reasoning tasks. You will break down the problem into smaller steps and solve each step methodically to arrive at the final answer.
        ").build();
    let agent = client.agent("gpt-4")
        .build();
    let chat_history: Vec<Message> = vec![
        Message {
            role: "user".to_string(),
            content: "机器人需要从网格的左上角导航到右下角。网格中有机器人无法通过的障碍物。机器人可以向上、向下、向左或向右移动。机器人到达目的地有多少条唯一的路径？
                    网格如下：
                    [
                        [0, 0, 0],
                        [0, 1, 0],
                        [0, 0, 0]
                    ]
                    其中0表示机器人可以通过的位置，1表示障碍物。".to_string(),
        },
        Message {
            role: "assistant".to_string(),
            content: "
            好的，我们可以通过分步骤的思维链来解决这个问题。目标是找到从网格的左上角 `(0, 0)` 到右下角 `(2, 2)` 的所有唯一路径，路径中不能重复访问任何一个位置，并且必须避开障碍物。

                ### 步骤 1：理解网格和障碍物

                首先，明确给定的网格：

                ```
                [
                    [0, 0, 0],
                    [0, 1, 0],
                    [0, 0, 0]
                ]
                ```

                - `0` 表示机器人可以通过的位置。
                - `1` 表示障碍物，机器人无法通过。

                因此，网格可以表示为：

                ```
                (0,0) (0,1) (0,2)
                (1,0) (1,1) (1,2)
                (2,0) (2,1) (2,2)
                ```

                其中 `(1,1)` 是障碍物。

                ### 步骤 2：确定起点和终点

                - **起点**： `(0, 0)`
                - **终点**： `(2, 2)`

                ### 步骤 3：列出可能的移动方向

                机器人可以向 **上、下、左、右** 移动，但不能离开网格边界，也不能进入障碍物或已经访问过的位置。

                ### 步骤 4：寻找所有可能的路径

                我们可以使用**深度优先搜索（DFS）**的方法来遍历所有可能的路径。以下是详细的路径探索过程：

                1. **路径 1**：
                    - 从 `(0,0)` 向 **右** 移动到 `(0,1)`
                    - 从 `(0,1)` 向 **右** 移动到 `(0,2)`
                    - 从 `(0,2)` 向 **下** 移动到 `(1,2)`
                    - 从 `(1,2)` 向 **下** 移动到 `(2,2)`
                    - **路径**： `(0,0) → (0,1) → (0,2) → (1,2) → (2,2)`

                2. **路径 2**：
                    - 从 `(0,0)` 向 **下** 移动到 `(1,0)`
                    - 从 `(1,0)` 向 **下** 移动到 `(2,0)`
                    - 从 `(2,0)` 向 **右** 移动到 `(2,1)`
                    - 从 `(2,1)` 向 **右** 移动到 `(2,2)`
                    - **路径**： `(0,0) → (1,0) → (2,0) → (2,1) → (2,2)`

                ### 步骤 5：验证是否有其他路径

                让我们检查是否存在其他可能的路径：

                - **尝试通过 `(1,1)`**：
                    - 无法通过，因为 `(1,1)` 是障碍物。
                
                - **尝试其他方向组合**：
                    - 例如，从 `(0,0)` 向下到 `(1,0)`，然后向右到 `(1,1)`（被阻挡）。
                    - 或者从 `(0,0)` 向右到 `(0,1)`，然后向下到 `(1,1)`（被阻挡）。

                通过上述分析，可以确认只有 **两条** 唯一的路径。

                ### 结论

                在给定的网格中，机器人从左上角 `(0, 0)` 导航到右下角 `(2, 2)` 的 **唯一路径数量为 2 条**。

                **具体路径如下**：

                1. `(0,0) → (0,1) → (0,2) → (1,2) → (2,2)`
                2. `(0,0) → (1,0) → (2,0) → (2,1) → (2,2)`".to_string(),
        },
    ];
    let question = "
        一个机器人需要从网格的左上角导航到右下角。网格中有机器人无法通过的障碍物。机器人可以向上、向下、向左或向右移动。机器人到达目的地有多少条唯一的路径？
        网格如下：
        [
            [0, 0, 1],
            [0, 1, 0],
            [0, 0, 0]
        ]
        其中0表示机器人可以通过的位置，1表示障碍物。
    ";
    let response = cot_agent.chat(question, chat_history).await?;
    // 提示词并获取响应
    // let response = cot_agent.prompt(question).await?;
    println!("Prompt: {}", question.replace("\\n", "\n"));
    println!("---------------------------------------------");
    println!("Response: {}", response.replace("\\n", "\n"));
    // println!("Response: {:?}", response);
    Ok(())
}

#[tokio::test]
async fn test_auto_CoT() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let client = create_client_from_env();
    let mut cot_agent = client.agent("gpt-4")
        .preamble("
            You are an intelligent assistant capable of performing complex reasoning tasks. You will break down the problem into smaller steps and solve each step methodically to arrive at the final answer.
        ").build();
    let example_question = "
        一个机器人需要从网格的左上角导航到右下角。网格中有机器人无法通过的障碍物。机器人可以向上、向下、向左或向右移动。机器人到达目的地有多少条唯一的路径？
        网格如下：
        [
            [0, 0, 1],
            [0, 1, 0],
            [0, 0, 0]
        ]
        其中0表示机器人可以通过的位置，1表示障碍物。
        请根据思维链一步步解决这个问题。
    ";
    let mut chat_history: Vec<Message> = vec![];
    let response = cot_agent.chat(example_question, chat_history.clone()).await?;
    println!("Prompt: {}", example_question.replace("\\n", "\n"));
    println!("---------------------------------------------");
    println!("Response: {}", response.replace("\\n", "\n"));
    chat_history.push(Message {
        role: "user".to_string(),
        content: example_question.to_string(),
    });
    chat_history.push(Message {
        role: "assistant".to_string(),
        content: response,
    });
    let question = "
        一个机器人需要从网格的左上角导航到右下角。网格中有机器人无法通过的障碍物。机器人可以向上、向下、向左或向右移动。机器人到达目的地有多少条唯一的路径？
        网格如下：
        [
            [0, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
            [1, 1, 0, 1, 0],
            [0, 0, 0, 0, 0]
        ]
        其中0表示机器人可以通过的位置，1表示障碍物。
    ";
    let response = cot_agent.chat(question, chat_history.clone()).await?;
    println!("Prompt: {}", question.replace("\\n", "\n"));
    println!("---------------------------------------------");
    println!("Response: {}", response.replace("\\n", "\n"));
    chat_history.push(Message {
        role: "user".to_string(),
        content: question.to_string(),
    });
    chat_history.push(Message {
        role: "assistant".to_string(),
        content: response,
    });
    let mes = Message {
        role: "user".to_string(),
        content: question.to_string(),
    };
    let serialized_history = serde_json::to_string(&chat_history)?;
    let mut file = File::create("chat_history.json")?;
    file.write_all(serialized_history.as_bytes())?;
    Ok(())
}