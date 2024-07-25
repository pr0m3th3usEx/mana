import numpy as np
import matplotlib.pyplot as plt

# Constants from the bonding curve
C = 32190005730
D = 1073000191
k = 30

# Bonding curve function
def token_amount(x):
    return D - C / (k + x)

# Derivative of bonding curve (price function) for x token sold
def token_price(x):
    return ((k + x) ** 2) / C

# Inverse bonding curve function
def sol_for_tokens(y):
    return (C / (D - y)) - k

# Function to simulate a buy order
def buy_order(pool_sol, pool_tokens, buy_amount_sol):
    new_sol = pool_sol + buy_amount_sol
    old_tokens = token_amount(pool_sol)
    new_tokens = token_amount(new_sol)

    print(f"old tokens: {old_tokens} - new tokens: {new_tokens}")
    tokens_bought =  new_tokens - old_tokens
    pool_tokens = pool_tokens - tokens_bought
    return new_sol, pool_tokens, tokens_bought

# Function to simulate a sell order
def sell_order(pool_sol, pool_tokens, tokens_sold):
    new_tokens = pool_tokens + tokens_sold
    old_sol = sol_for_tokens(pool_tokens)
    new_sol = sol_for_tokens(new_tokens)
    sol_received = old_sol - new_sol
    return new_sol, new_tokens, sol_received

# Simulate a series of trades
trades = [
    {'type': 'buy', 'amount': 0.00001},
    {'type': 'buy', 'amount': 0.00001},
    {'type': 'buy', 'amount': 0.00001},
    {'type': 'buy', 'amount': 0.00001},
    {'type': 'buy', 'amount': 0.00001},
    {'type': 'buy', 'amount': 0.00001},
    {'type': 'buy', 'amount': 0.00001},
    {'type': 'buy', 'amount': 0.00001},
    {'type': 'buy', 'amount': 0.00001},
    # {'type': 'sell', 'amount': 5e7},  # Amount of tokens sold
    # {'type': 'buy', 'amount': 0.000000056},
    # {'type': 'sell', 'amount': 1e8}   # Amount of tokens sold
]

# Initialize pool state
pool_sol = 0  # Starting with no SOL in the pool
pool_tokens = initial_tokens = 1_000_000_000  # Starting with 1 billion tokens

# Execute trades and track prices
trade_results = []

for trade in trades:
    action = ""
    if trade['type'] == 'buy':
        pool_sol, pool_tokens, tokens_bought = buy_order(pool_sol, pool_tokens, trade['amount'])
        action = f"Bought {tokens_bought:.10f} tokens for {trade['amount']} SOL"
    elif trade['type'] == 'sell':
        pool_sol, pool_tokens, sol_received = sell_order(pool_sol, pool_tokens, trade['amount'])
        action = f"Sold {trade['amount']} tokens for {sol_received:.10f} SOL"

    # Calculate current token price
    current_price = token_price(pool_sol)
    trade_results.append((trade['type'], trade['amount'], pool_sol, pool_tokens, current_price, action))

# Print the results
for result in trade_results:
    trade_type, amount, sol, tokens, price, action = result
    print(f"{action} -> Pool SOL: {sol}, Pool Tokens: {tokens}, Price per Token: {price:.10f} SOL")

# Plot the evolving price of the token
sol_values = np.linspace(0, 100, 400)
token_prices = [token_price(sol) for sol in sol_values]
token_amounts = [token_amount(sol) for sol in sol_values]

print(f"Pr: {token_price( 1.021 ):.10f}")
# print(f"Price per token: {sol_for_tokens(1)}")

# plt.figure(figsize=(10, 6))
# plt.plot(sol_values, token_prices, label='Token Price (SOL)')
# plt.plot(sol_values, token_amounts, label='Token Price (SOL)')
# plt.xlabel('Total $SOL in Pool')
# plt.ylabel('number of Token obtained')
# plt.title('Evolving Price of Token')
# plt.legend()
# plt.grid(True)
# plt.show()
