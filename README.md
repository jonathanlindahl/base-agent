# base-agent
simple ai agent foundation to be used in other projects

# setup

# usage
If using a model installed through ollama, the model name needs to be specified in the request body of the call function in llm.rs. At the time of writing, this is phi:mini, which is very light weight to save on memory. With access to better hardware, it's recommended to switch to a different model like llama3 or possibly phi3.
