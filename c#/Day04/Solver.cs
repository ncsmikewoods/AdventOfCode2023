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
        var scores = _cards.Select(card => card.CalculateScore()).ToList();

        foreach (var score in scores)
        {
            Console.WriteLine($"Score - {score}");
        }

        return scores.Sum();
    }
    
    public int Solve2()
    {
        return 1;
    }

    void GetInputs()
    {
        var lines = File.ReadLines("input.txt").ToList();
        _cards = lines.Select(line => new Card(line)).ToList();
    }
}