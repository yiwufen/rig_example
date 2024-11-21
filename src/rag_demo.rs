use rig::{completion::Prompt, embeddings::EmbeddingsBuilder, vector_store::{in_memory_store::InMemoryVectorStore, VectorStore}};

use crate::untils::create_client_from_env;


#[tokio::test]
async fn test_rag() -> Result<(), anyhow::Error> {
    // 加载环境变量
    dotenv::dotenv().ok();
    
    // 根据环境变量创建 OpenAI 客户端
    let client = create_client_from_env();
    
    // 获取嵌入模型
    let embedding_model = client.embedding_model("text-embedding-ada-002");
    
    // 创建一个内存向量存储
    let mut store = InMemoryVectorStore::default();
    
    // 构建嵌入文档
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .simple_document("doc0", "Definition of a *flurbo*: A flurbo is a green alien that lives on cold planets")
        .simple_document("doc1", "Definition of a *glarb-glarb*: A glarb-glarb is a ancient tool used by the ancestors of the inhabitants of planet Jiro to farm the land.")
        .simple_document("doc2", "Definition of a *linglingdong*: A term used by inhabitants of the far side of the moon to describe humans.")
        .build()
        .await?;
    
    // 将嵌入文档添加到存储中
    store.add_documents(embeddings).await?;

    // 创建索引
    let index = store.index(embedding_model);

    // 创建 RAG 代理
    let rag_agent = client.agent("gpt-4")
        .preamble("
            You are a dictionary assistant here to assist the user in understanding the meaning of words.
            You will find additional non-standard word definitions that could be useful below.
        ")
        .dynamic_context(1, index)
        .build();

    // 提示词并获取响应
    let response = rag_agent.prompt("What does \"glarb-glarb\" mean?").await?;
    println!("Response: {:?}", response);

    Ok(())
}

#[tokio::test]
async fn test_no_rag() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let client = create_client_from_env();
    let agent = client.agent("gpt-4").build();
    let prompt = "What does \"glarb-glarb\" mean?";
    let response = agent.prompt(prompt).await?;
    println!("Response: {:?}", response);
    Ok(())
}