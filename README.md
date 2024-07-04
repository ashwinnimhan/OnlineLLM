##### PROJECT FILES / MODULES
| Component | Description |
|-------------------|-------------|
| bing_search | Search call to Bing News engine and results parsing |
| yahoo_search | Search call to Yahoo News engine and results parsing |
| google_search | Search call to Google News engine and results parsing |
| web_crawler | Crawls links consolidated from search result pages |
| llm_request | OpenAI GPT-3.5-turbo call and response parsing |
| common | All commonly shared structs/custom data types |
| request_handler | Request pipeline manager (query -> search -> rerank -> prompt -> response) |
| main | routes mapping and webserver |

##### RESEARCH (Stages of Online LLM Response Generation)
1. **Question Analysis and Context Retrieval**
	* Question Categorization and Premise Check `Not Done`
	* In-Context Learning via Search Engine Responses<sup>[1]</sup> `Done - Google, Yahoo, Bing but currently utilizing Yahoo and Bing due to Google Search limitations`
2. ** Context Processing and Optimization**
	* Metadata (title, publisher, timestamp, URL, snippet) extraction `Done`
	* Reranking of Context Chunks to alleviate loss in the middle issue `Done - simple rerank based on published time`
	* Prompt compression and Adaptive granular control over context compression <sup>[2]</sup> `Not Done`
3. **Reasoning Approaches** `Not Done`
	* Single-hop Reasoning
	* Multi-hop Reasoning
		* Few-shot Prompting
		* Iterative Retriever, Reader, and Reranker <sup>[3]</sup>
		* Self-Ask: Generate follow-up questions <sup>[4]</sup>
		* DEMONSTRATE–SEARCH–PREDICT <sup>[5]</sup>


##### CURRENT SYSTEM -> PROPOSED IMPROVEMENTS
1. **Architecture**
	 * The code runs asynchronously on a single EC2 t3.xlarge instance. The current Rust implementation limits access to a well-developed ecosystem of libraries supporting Web Parsing in languages like Python / JS. Adopting a microservices architecture could enable the use of diverse tools and components optimized for specific pipeline tasks, while avoiding compatibility management issues.
2. **NLP**
	* The system currently uses verbatim prompt as a search query. To improve, we can implement query expansion techniques like using retrieved documents to reformulate queries, context summarization with relevance-based compression or traditional NLP techniques like Entity Recognition/Aspect Identification, etc. for chat prompts.
3. **Crawler**
	* The web crawler can be enhanced by implementing better parsing rules and extracting semantic tags from crawled content. Additionally, a blocklist for domains that prohibit crawling should be established to avoid futile attempts. Local indexing and Retrieval-Augmented Generation (RAG) based on crawled documents can also boost performance.
4. **Prompt Engineering**
	* Few-shot learning, context-based custom instructions, and query intent categorization can enhance prompt effectiveness, leading to more concise and relevant responses while reducing hallucinations.
5. **Language Model**
	* Apart from training from scratch or fine-tuning, Small Language Models (SLMs) can be used for compression or summarization tasks to reduce costs and ensure strong domain alignment.		

[1]: https://arxiv.org/pdf/2310.03214
[2]: https://www.llamaindex.ai/blog/longllmlingua-bye-bye-to-middle-loss-and-save-on-your-rag-costs-via-prompt-compression-54b559b9ddf7
[3]: https://arxiv.org/pdf/2010.12527
[4]: https://arxiv.org/pdf/2210.03350
[5]: https://arxiv.org/pdf/2212.14024
