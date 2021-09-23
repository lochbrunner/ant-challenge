from environment.api import Action, Activity, BaseAnt, Perception


class MyAnt(BaseAnt):
    def think(self, perception: Perception) -> Action:
        # speed control
        if perception.velocity > 1:
            accelerate = -0.1
        else:
            accelerate = 0.1

        # summary
        return Action(turn=0, accelerate=accelerate, activity=Activity.NONE)
