namespace Day04;

public class Solver
{
    private List<Card> _cards;

    public Solver()
    {
        GetInputs();
    }

    public int Solve1()
    {
        var winnerCounts = _cards.Select(card => card.CountWinners()).ToList();

        return winnerCounts
            .Select(x => (int)Math.Pow(2, x - 1))
            .Sum();
    }
    
    public int Solve2()
    {
        for (var i = 0; i < _cards.Count; i++)
        {
            var winnerCount = _cards[i].CountWinners();
            foreach (var j in Enumerable.Range(i+1, winnerCount))
            {
                _cards[j].Multiplier += _cards[i].Multiplier;
            }
        }
        
        return 
            _cards
                .Select(c => c.Multiplier)
                .Sum();
    }

    void GetInputs()
    {
        var lines = File.ReadLines("input.txt").ToList();
        _cards = lines.Select(line => new Card(line)).ToList();
    }
}