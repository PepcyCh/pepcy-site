---
title: 强化学习 - Policy Iteration & Value Iteration
date: 2020-03-03 12:41:19
tags: [课程笔记, 实现笔记, 强化学习]
categories: 学习笔记（大学课外）
---

近日处于兴趣，在看 [UCL 的 强化学习](http://www0.cs.ucl.ac.uk/staff/d.silver/web/Teaching.html)，目前看完了前三节。第一节是绪论，第二节介绍各种概念，第三届讲 Policy Iteration 和 Value Iteration。看完第三节后觉得应该写一下，所以对课件中提到的借车问题，用 C++ 分别实现了一下。笔记和代码会放在 GitHub 上：[RL-UCL-notes](https://github.com/PepcyCh/UCL-reinforcement-learning-notes)。

部分有参考这个 repo：[ShangtongZhang/reinforcement-learning-an-introduction](https://github.com/ShangtongZhang/reinforcement-learning-an-introduction)

## 借车问题

有两处停车场，每个停车场的容量是 20 辆。每次你可以把至多 5 辆车从其中一个停车场移至另一个，移动一辆车的代价是 2 点。同时，每次会有若干人在停车场租车与还车，租出一辆车的收益是 10 点。每次租车与还车的数目服从泊松分布，停车场一的租车与还车的 $\lambda$ 值均为 3，停车场二租车的 $\lambda$ 值为 4、还车为 2。你要制定策略最大化收益。

<!-- more -->

## 基本概念

整个活动是一个马尔可夫决策过程（Markov Decision Process，MDP），一个 MDP 可由元组 $\lang \mathcal{S}, \mathcal{A}, \mathcal{P}, \mathcal{R}, \gamma \rang$ 表示。此外，还有一些重要概念，如策略、价值函数、Bellman 方程等。接下来会大概地描述一下它们的含义。想要具体了解其准确含义，还请去看原课件或书。

### 状态 state $\mathcal{S}$

以借车问题为例，状态就是两个停车场的车数，总共有 $21 \times 21 = 441$ 个。

在其他的例子中，如走迷宫就是指所在地，井字棋就是棋盘上的样子。

### 行动 action $\mathcal{A}$

以借车问题为例，行动就是移动车辆。注意，租车和还车不是行动，因为它们不是你要决定的。

在其他的例子中，如走迷宫就是往哪个方向走，井字棋就是在哪里落子。

### 转移 transition $\mathcal{P}$

转移是从一个状态到另一状态的概率，具体地说，用 $\mathcal{P_{ss’}^a}$ 表示在状态 $s$，使用行动 $a$ 后能到达状态 $s’$ 的概率。

在走迷宫的例子中，这个概率只会是 $0, 1$ 两个值。而借车问题中，由于租车和还车是随机的，这个概率就不是二值化的，可以由泊松分布的式子计算出来。

### 收益 reward $\mathcal{R}$

收益可以是借车问题中这样直接描述出来的。也可以是根据问题自行定义的值，如走迷宫可以定义走一步的收益是 $-1$。

用 $\mathcal{R_s^a}$ 表示在状态 $s$，使用行动 $a$ 后会获得的收益的期望。

### 衰减因子 $\gamma$

$\gamma$ 是一个在 $[0, 1]$ 范围内的值，越接近 1，表示越考虑将来；越接近 0，表示越注重眼前。

### 回报 return $G_t$

第 $t$ 步的回报 $G_t$ 定义为 $R_{t + 1} + \gamma R_{t + 2} + \gamma^2 R_{t + 3} + \cdots$

其中 $R_t$ 表示第 $t$ 步时获得的收益。

根据定义，可以看到衰减因子的作用为何会体现为这样。

### 策略 policy $\pi(a \mid s)$

策略是行动的分布，指在状态 $s$ 下，要使用行动 $a$ 的概率。

### 价值函数 value function

有状态价值函数（state-value function）和行动价值函数（action-value function）：

state-value function $v_\pi(s)$ 表示使用策略 $\pi(a \mid s)$ ，在状态 $s$ 时，会得到的回报的期望。

action-value function $q_\pi(s, a)$ 表示使用策略 $\pi(a \mid s)$ ，在状态 $s$ 时使用行动 $a$，会得到的回报的期望。

### Bellman 方程

Bellman 方程把回报分为立即的回报 $R_{t + 1}$ 和将来的衰减的回报。

对于价值函数，将来的回报就是 $\gamma v(S_{t + 1})$，结合价值函数的定义和一些概率上的运算，推导出了 $v_\pi(s) = \mathcal{R_s^\pi} + \gamma \sum_{s’ \in \mathcal{S}} \mathcal{P_{ss’}^\pi} v_\pi(s’)$。

把策略 $\pi$ 拆成行动 $a$ 的形式就是：
$$
v_\pi(s) = \sum_{a \in \mathcal{A}} \pi(a \mid s) \left( \mathcal{R_s^a} + \gamma \sum_{s' \in \mathcal{S}} \mathcal{P_{ss'}^a} v_\pi(s') \right)
$$
（行动价值函数也有相应的 Bellman 方程，不过接下来都会用的是状态价值函数，就不提了。）

有了 Bellman 方程，在已知 MDP 的收益和转移、决定了策略后就可以算出价值函数。

## 策略迭代 Policy Iteration

### 基本思想

无论是策略迭代还是价值迭代，都需要知道 MDP 的转移。

这样的话，有了 Bellman 方程，我们就可以求出价值函数。

一个直观的想法是写出矩阵形式然后求解矩阵方程，不过这只适用于状态数较小的情况（其实借车问题的状态数 441 用解矩阵方程还是可以接受的）。更大一些的状态集可以用迭代求解。这个步骤被称为策略评价（policy evaluation）。

在求出价值后，我们可以得到一种贪心策略，即每次贪心地选择价值最大的次态，这被称作策略改进（policy improvement）。事实上，不只贪心可以做策略改进。

策略迭代的想法是，我们一开始的策略是等概率随机，之后不断地进行策略评价和改进，最终就会得到最优策略。而这一点被证明是正确的。

### 代码实现

首先，根据策略迭代的基本思想，可以写出主要函数的一个框架：

```c++
void Train() {
    while (true) {
        PolicyEvaluation()；
        int n_policy_changed = PolicyImprovement();
        if (n_policy_changed == 0)
            break;
    }
}
```

策略评价和改进部分，都是对于每一个状态做一些相同事情，所以可以写成：

```c++
void PolicyEvaluation() {
    while (true) {
        Values new_values;
        Reward diff = 0;
        for (const auto &state : all_states) {
            Reward new_value = CalcPE(state);
            diff += std::abs(new_value - values[state]);
            new_values[state] = new_value;
        }
        values = new_values;
        if (diff < eps)
            break;
    }
}

int PolicyImprovement() {
    Policies new_policies;
    int diff = 0;
    for (const auto &state : all_states) {
        Policy new_policy = DoPI(state);
        diff += 1 - (policies[state] == new_policy);
        new_policies[state] = new_policy;
    }
    policies = new_policies;
    return diff;
}
```

`CalcPE(state)` 可以根据 Bellman 方程写出来：

```c++
Reward CalcPE(const State &state) const {
    const Policy &policy = policies.at(state);

    Reward res = 0;
    for (const auto &[action, prob] : policy) {
        Reward imm_reward = CalcImmReward(state, action);
        Reward dis_reward = CalcDisReward(state, action);
        res += (imm_reward + dis_reward * gamma) * prob;
    }
    return res;
}
```

（其中 `imm_reward` 是想表示 immediate reward，`dis_reward` 是想表示 discounted reward）

`CalcImmReward(state, action)` 就是 $\mathcal{R_s^a}$，根据具体的问题有所不同。

`CalcDisReward(state, action)` 则可以继续写下去：

```c++
Reward CalcDisReward(const State &state, const Action &action) const {
    Reward res = 0;
    for (const auto &state_p : all_states) {
        res += values.at(state_p) * CalcPss(state, state_p, action);
    }
    return res;
}
```

`CalcPss(state, state_p, action)` 就是 $\mathcal{P_{ss’}^a}$ ，根据具体的问题有所不同。

策略改进部分，根据价值贪心还是很好写的：

```c++
Policy DoPI(const State &state) const {
    Reward max = std::numeric_limits<Reward>::lowest();
    int n_max = 0;
    std::unordered_map<Action, Reward> temp_map;

    for (const auto &action : all_actions) {
        if (!state.CanDoAction(action))
            continue;
        Reward value = CalcDisReward(state, action);
        temp_map[action] = value;
        if (value > max) {
            max = value;
            n_max = 1;
        } else if (value == max) {
            ++n_max;
        }
    }

    Policy new_policy;
    double prob = 1.0 / n_max;
    for (const auto &[action, reward] : temp_map) {
        if (reward == max) {
            new_policy[action] = prob;
        }
    }
    return new_policy;
}
```

以上部分对于任何满足条件的问题都是适用的，所以我把它们放进了一个类中，继承它的类需要根据具体问题实现 `CalcPss()`  和 `CalcImmReward()`。

代码中的 `State`、`Action` 也是依赖具体问题的，所以做成了模版参数。其余的 `Reward`、`Policy` 等都是一些别名：

```c++
using States = std::unordered_set<State, StateHash>;
using Actions = std::unordered_set<Action, ActionHash>;
using Reward = double;
using Values = std::unordered_map<State, Reward, StateHash>;
using Policy = std::unordered_map<Action, double, ActionHash>;
using Policies = std::unordered_map<State, Policy, StateHash>;
```

对于具体的借车问题，其状态就是一个数对表示车数，行动用 $[-5, 5]$ 的整数表示，正数表示从停车场一运到停车场二。

在实现具体借车问题的 `CalcPss()` 和 `CalcImmReward()` 时，概率的计算部分是用了比较暴力的做法，直接枚举借车数与还车数，我是按照先借车不能借到负数，后还车不能还超容量枚举，这样算出的概率和实际也会有一定偏差，不过跑出来的结果看起来还是可以的。

```c++
double CalcPss(const State &state, const State &state_p,
        const Action &action) const {
    if (!state.CanDoAction(action))
        return 0;

    int new_x = std::min(state.x - action, kMaxCars);
    int new_y = std::min(state.y + action, kMaxCars);
    int delta_x = state_p.x - new_x;
    int delta_y = state_p.y - new_y;

    double pss = 0;
    for (int req1 = std::max(0, -delta_x); req1 <= new_x; req1++) {
        for (int req2 = std::max(0, -delta_y); req2 <= new_y; req2++) {
            int ret1 = req1 + delta_x;
            int ret2 = req2 + delta_y;
            double prob = Poisson(req1, kAvgReq1) * Poisson(ret1, kAvgRet1)
                * Poisson(req2, kAvgReq2) * Poisson(ret2, kAvgRet2);
            pss += prob;
        }
    }
    assert(pss >= 0);
    return pss;
}

Reward CalcImmReward(const State &state, const Action &action) const {
    if (!state.CanDoAction(action))
        return -1.0 / 0.0;
    Reward move_reward = kMoveCost * std::abs(action);

    Reward rent_reward = 0;
    int new_x = std::min(state.x - action, kMaxCars);
    int new_y = std::min(state.y + action, kMaxCars);
    for (int req1 = 0; req1 <= new_x; req1++) {
        for (int req2 = 0; req2 <= new_y; req2++) {
            double prob_req = Poisson(req1, kAvgReq1) * Poisson(req2, kAvgReq2);
            double prob_ret = 0;
            for (int ret1 = 0; new_x - req1 + ret1 <= kMaxCars; ret1++) {
                for (int ret2 = 0; new_y - req2 + ret2 <= kMaxCars; ret2++) {
                    double prob = Poisson(ret1, kAvgRet1) *
                        Poisson(ret2, kAvgRet2);
                    prob_ret += prob;
                }
            }
            rent_reward += prob_req * prob_ret * (req1 + req2) * kRentReward;
        }
    }

    return move_reward + rent_reward;
}
```

## 价值迭代 Value Iteration

### 基本思想

每次迭代，我们直接选取让回报最大的行动更新价值，这就是价值迭代。写成式子就是：
$$
v(s) = \max_{a \in \mathcal{A}} \left( \mathcal{R_s^a} + \gamma \sum_{s' \in \mathcal{S}} \mathcal{P_{ss'}^a} v(s') \right)
$$
价值迭代的思想就是，反复迭代下去会得到最优策略。同样，这被证明是正确的。

### 代码实现

价值迭代与策略迭代有很大的相似性，比如相似的主框架：

```c++
void Train() {
    while (true) {
        double diff = BellmanOptimality();
        if (diff < eps)
            break;
    }

    policies = GetPoliciesFromValues();
}
```

`BellmanOptimality()` 也同样是对每个状态做一些相同的事情：

```c++
Reward BellmanOptimality() {
    Values new_values;
    Reward diff = 0;
    for (const auto &state : all_states) {
        Reward new_value = CalcOptimality(state);
        diff += std::abs(new_value - values[state]);
        new_values[state] = new_value;
    }
    values = new_values;
    return diff;
}
```

真正的计算函数也很直白：

```c++
Reward CalcOptimality(const State &state) const {
    Reward max = std::numeric_limits<Reward>::lowest();
    for (const auto &action : all_actions) {
        if (state.CanDoAction(action)) {
            Reward imm_reward = CalcImmReward(state, action);
            Reward dis_reward = CalcDisReward(state, action);
            Reward reward = imm_reward + dis_reward * gamma;
            max = std::max(max, reward);
        }
    }
    return max;
}
```

`CalcImmReward()` 和 `CalcDisReward()` 都是和策略迭代一样的。

而最后的 `GetPoliciesFromValues()` 就是策略迭代的策略改进。

可以发现，用于策略迭代的具体问题的 `CalcPss()` 和 `CalcImmReward()` 完全不需要修改。

## 实现并行

在借车问题中，策略迭代应该是跑了大概 1900s 的样子，价值迭代跑了大概 2200s。再看一下 `PolicyEvaluation()`、`PolicyImprovement()`、`BellmanOptimality()`、`GetPoliciesFromValues()`。它们都是对每一个状态做一些一样的事情，这些事情对公用的数据都是只读不写，不同状态之间相互独立，很容易改成并行的样子。

实现中，使用 `std::vector<std::future<T>>` 来处理，以价值迭代为例：

```c++
Reward BellmanOptimality() {
    Values new_values;
    Reward diff = 0;
    std::vector<std::future<std::pair<Reward, State>>> handles;
    for (const auto &state : all_states) {
        handles.push_back(std::async(std::launch::async,
                [this, &state]() { return this->CalcOptimality(state); }));
    }
    for (auto &future : handles) {
        auto [new_value, state] = future.get();
        diff += std::abs(new_value - values[state]);
        new_values[state] = new_value;
    }
    values = new_values;
    return diff;
}
```

要让 `CalcOptimality(state)` 再把 `state` 返回来。可以看出来，对代码的改动很小。

实测中，并行的价值迭代跑了 450s 左右。