# OnlineLLM
Repo for an Online LLM implemented in Rust Lang

### Question Answering and Reasoning Techniques
Here is some of the research that helped inform my decisions on the system architecture.

#### 1. Question Analysis and Context Retrieval
- Question Categorization and Premise Check
- In-Context Learning via Search Engine Responses[^1]

#### 2. Context Processing and Optimization
- Metadata extraction
- Reranking of Context Chunks to alleviate loss in the middle issue
- Prompt compression and Adaptive granular control during compression[^2]

#### 3. Reasoning Approaches
- Single-hop Reasoning
- Multi-hop Reasoning
    * Few-shot Prompting
    * Iterative Retriever, Reader, and Reranker[^3]
    * Self-Ask: Generate follow-up questions[^4]
    * DEMONSTRATE–SEARCH–PREDICT[^5]

[^1]: https://arxiv.org/pdf/2310.03214
[^2]: https://www.llamaindex.ai/blog/longllmlingua-bye-bye-to-middle-loss-and-save-on-your-rag-costs-via-prompt-compression-54b559b9ddf7
[^3]: https://arxiv.org/pdf/2010.12527
[^4]: https://arxiv.org/pdf/2210.03350
[^5]: https://arxiv.org/pdf/2212.14024