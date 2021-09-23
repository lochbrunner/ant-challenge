# Environment

## Features

1. Populate
1. Sense
1. Call agents
1. Simulate next step


## User Agents

The user has to implement the following interface

```python
class BaseAnt(ABC):
    @abstractmethod
    def think(self, perception: Perception) -> Action:
        pass
```

for each ant.