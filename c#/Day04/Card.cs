namespace Day04;

public class Card
{
    public Card(string line)
    {
        var delims = new char[] { ':', '|' };
        var tokens = line.Split(delims, StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);

        Name = tokens[0];
        WinningNumbers = tokens[1].
            Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(int.Parse).ToList();
        MyNumbers = tokens[2].
            Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(int.Parse).ToList();
    }

    public string Name { get; set; }
    public List<int> WinningNumbers { get; set; }
    public List<int> MyNumbers { get; set; }

    public int CalculateScore()
    {
        var winningPicks = MyNumbers.Where(myNum => WinningNumbers.Contains(myNum)).ToList();
        return (int)Math.Pow(2, winningPicks.Count - 1);
    }
}