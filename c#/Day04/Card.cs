namespace Day04;

public class Card
{
    public Card(string line)
    {
        var delims = new char[] { ':', '|' };
        var tokens = line.Split(delims, StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);

        Name = tokens[0];
        CardId = int.Parse(tokens[0].Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)[1]);
        WinningNumbers = tokens[1].
            Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(int.Parse).ToList();
        MyNumbers = tokens[2].
            Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(int.Parse).ToList();
    }

    public string Name { get; set; }
    public int CardId { get; set; }
    public List<int> WinningNumbers { get; set; }
    public List<int> MyNumbers { get; set; }
    public int Multiplier { get; set; } = 1;

    // public int CalculateScore()
    // {
    //     var winningPicks = MyNumbers.Where(myNum => WinningNumbers.Contains(myNum)).ToList();
    //     return (int)Math.Pow(2, winningPicks.Count - 1);
    // }

    public int CountWinners()
    {
        return MyNumbers.Count(myNum => WinningNumbers.Contains(myNum));
    }
}