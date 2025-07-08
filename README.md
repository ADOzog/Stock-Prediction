# Overview
I want to use my knowledge of Stochastic Processes and Rust to make a predictive model for pricing stocks.  


### Implementation Steps

1. **Data Handling**
   - Use API calls to fetch historical stock data and real time data.
   - Implement a database to ease this process.
      - Need to understand which is best for this and learn how it works too. 

2. **Model Development**
   - Implement the GBM model.
   - Write functions to estimate parameters, simulate price paths, and visualize results.
      - I have much to understand for this step.

3. **Testing and Validation**
   - Test the model with different stocks and time frames to ensure robustness.
   - Validate the model's predictions against actual market data.
      - This can be done with historic data rapidly or real time data slowly.

4. **Documentation**
   - Document code and findings well.
   - Talk to professionals about what can be done to improve this. 


### Key Components
1. **Understanding Geometric Brownian Motion (GBM)**
   - **Mathematical Representation**: The GBM model is defined by the stochastic differential equation:
     $$dS_t = \mu S_t dt + \sigma S_t dW_t$$
     where:
     - $ S_t $ is the stock price at time $ t $.
     - $ \mu $ is the expected return (drift).
     - $ \sigma $ is the volatility (standard deviation of returns).
     - $ dW_t $ is a Wiener process (Brownian motion).

2. **Data Collection**
   - Gather historical stock price data for the chosen stock or index. This data can be obtained from financial APIs (Yahoo Finance).
   - Ensure the data includes timestamps, opening prices, closing prices, high and low prices, and volume.
   - The crate I found already implements most of this.

3. **Model Implementation**
   - **Parameter Estimation**: Calculate the parameters $ \mu $ and $ \sigma $ from the historical data:
     - $ \mu $ can be estimated as the average of the logarithmic returns.  
     - $ \sigma $ can be estimated as the standard deviation of the logarithmic returns.  
   - **Simulation**: Use the estimated parameters to simulate future stock price paths using the GBM formula. This can be done using numerical methods to discretize the stochastic differential equation.  

4. **Prediction and Visualization**
   - Generate multiple simulated price paths to visualize the potential future movements of the stock price.  
   - Plot the historical prices along with the simulated paths to provide context.  
   - Calculate and visualize confidence intervals for the predicted prices.  

5. **Backtesting and Evaluation**
   - Compare the predicted prices with actual future prices (if available) to evaluate the model's accuracy.  
   - Use metrics such as Mean Absolute Error (MAE) or Root Mean Squared Error (RMSE) to quantify prediction performance.  
   - Would a Monte-Carlo Simulation be a good way to test the model? 

### Things that I need to learn 

1. How are $ \mu $ and $ \sigma $ found? And what are the best approaches to find them?  
    - This appears to be the most important part of the whole project.

2. What is the best place to get financial data? Are the ones I have listed good enough?  
    - Yahoo Finance seems to be good enough

3. What makes good backtests, how much testing is needed, and how long does the testing need?  


#### Ideas to come back to 

1. Exponential moving average gives more weight to recent observations, making it more responsive to new information. The formula for calculating the EMA is: EMA_t=α⋅R_t + (1−α)⋅EMA_{t−1} Where α is the smoothing factor (between 0 and 1) that determines the weight given to the most recent return.
    - Some form of moving window can also be used to keep the data more relavant.

2. Do I want to use GMB or a different optimization approach?

3. Embeddings for news documents or RAG to summerize them
    - There is support for news in the yahoo API crate and I just have to figure out how to use it. The embedding vector could be some kind of input to an optomiztion/prediction model.



#### Crates to use

- diffusionx
    - This will do the heavy lifting for the GBM

- serde
    - To parse the API data

- nalgebra 
    - Linear Algebra management 

- yahoo_finance_api
    - To do al lthe calls to Yahho Finance, its already implemented

